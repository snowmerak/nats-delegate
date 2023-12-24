use std::{error::Error, collections::HashMap};
use crate::callback::SubscriptionCallback;

pub struct Delegate {
    conn: nats::Connection,
    subscribes: HashMap<String, nats::Subscription>,
}

impl Delegate {
    pub fn connect(address: &str, username: &str, passworld: &str) -> Result<Self, Box<dyn Error>> {
        let conn = match nats::Options::with_user_pass(username, passworld)
        .connect(address) {
            Ok(conn) => conn,
            Err(e) => return Err(Box::new(e)),
        };

        Ok(Delegate {
            conn: conn,
            subscribes: HashMap::new(),
        })
    }

    pub fn publish(&self, subject: &str, message: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        match self.conn.publish(subject, message) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn subscribe(&mut self, subject: &str, callback: Box<dyn SubscriptionCallback>) -> Result<(), Box<dyn Error>> {
        let sub = match self.conn.subscribe(subject) {
            Ok(sub) => sub,
            Err(e) => return Err(Box::new(e)),
        };

        match self.subscribes.insert(subject.to_string(), sub) {
            Some(_) => {},
            None => {},
        }

        let borrowed_sub = self.subscribes.get(subject).unwrap().clone();

        tokio::spawn(async move {
            loop {
                let msg = match borrowed_sub.next() {
                    Some(msg) => msg,
                    None => break,
                };

                let response = callback.on_subscription(&msg, msg.reply.is_some());

                match msg.reply {
                    Some(reply) => {
                        match response {
                            Some(response) => {
                                match msg.respond(response) {
                                    Ok(_) => {},
                                    Err(e) => println!("Failed to publish response: {}", e),
                                }
                            },
                            None => {
                                match msg.respond(&vec![]) {
                                    Ok(_) => {},
                                    Err(e) => println!("Failed to publish response: {}", e),
                                }
                            },
                        }
                    },
                    None => {},
                }
            }
        });

        Ok(())
    }

    pub async fn unsubscribe(&mut self, subject: &str) -> Result<(), Box<dyn Error>> {
        match self.subscribes.remove(subject) {
            Some(sub) => {
                match sub.drain() {
                    Ok(_) => {},
                    Err(e) => return Err(Box::new(e)),
                }
                match sub.unsubscribe() {
                    Ok(_) => {},
                    Err(e) => return Err(Box::new(e)),
                }
                Ok(())
            },
            None => Ok(()),
        }
    }

    pub async fn request(&self, subject: &str, message: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        match self.conn.request(subject, message) {
            Ok(msg) => Ok(msg.data),
            Err(e) => Err(Box::new(e)),
        }
    }
}