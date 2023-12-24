use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::protocol::Message;

const BROADCASTER_CHANNEL_SIZE: usize = 100;

pub struct Broadcaster {
    pub senders: RwLock<HashMap<String, tokio::sync::broadcast::Sender<Message>>>,
}

impl Broadcaster {
    pub fn new() -> Self {
        Broadcaster {
            senders: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register(&mut self, subject: &str) -> tokio::sync::broadcast::Sender<Message> {
        let senders = self.senders.read().await;
        if senders.contains_key(subject) {
            return senders.get(subject).unwrap().clone();
        }
        drop(senders);

        let (sender, _) = tokio::sync::broadcast::channel(BROADCASTER_CHANNEL_SIZE);

        self.senders.write().await.insert(subject.to_string(), sender.clone());

        sender
    }

    pub async fn publish(&self, subject: &str, message: Message) {
        match self.senders.read().await.get(subject) {
            Some(sender) => {
                match sender.send(message) {
                    Ok(_) => {},
                    Err(e) => println!("Failed to publish message: {}", e),
                }
            },
            None => {},
        }
    }

    pub async fn unregister(&mut self, subject: &str) {
        match self.senders.write().await.remove(subject) {
            Some(_) => {},
            None => {},
        }
    }

    pub async fn subscribe(&mut self, subject: &str) -> Option<tokio::sync::broadcast::Receiver<Message>> {
        match self.senders.write().await.get(subject) {
            Some(sender) => {
                Some(sender.subscribe())
            },
            None => {
                None
            },
        }
    }
}
