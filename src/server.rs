use std::{error::Error, sync::Arc};

use tokio::{net::TcpListener, spawn};

use crate::{delegate::Delegate, broadcaster::Broadcaster, protocol::Message};


pub struct TcpServer {
    pub nats_addresses: Vec<String>,
    pub nats_username: String,
    pub nats_password: String,
    pub server_address: String,
    listener: Option<TcpListener>,
    delegate: Option<Delegate>,
    broadcaster: Option<Broadcaster>,
}

impl TcpServer {
    pub fn new(nats_addresses: Vec<&str>, nats_username: &str, nats_password: &str, server_address: &str) -> Arc<Self> {
        Arc::new(TcpServer {
            nats_addresses: nats_addresses.iter().map(|s| s.to_string()).collect(),
            nats_username: nats_username.to_string(),
            nats_password: nats_password.to_string(),
            server_address: server_address.to_string(),
            listener: None,
            delegate: None,
            broadcaster: None,
        })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let listener = match TcpListener::bind(self.server_address.as_str()).await {
            Ok(listener) => listener,
            Err(e) => return Err(e.into()),
        };

        self.listener = Some(listener);

        let delegate = match Delegate::connect(self.nats_addresses.join(",").as_str(), self.nats_username.as_str(), self.nats_password.as_str()) {
            Ok(delegate) => delegate,
            Err(e) => return Err(e.into()),
        };

        self.delegate = Some(delegate);

        let broadcaster = Broadcaster::new();

        self.broadcaster = Some(broadcaster);

        spawn(async move {
            loop {
                let (mut stream, _) = match listener.accept().await {
                    Ok((stream, _)) => (stream, None),
                    Err(e) => {
                        println!("Failed to accept connection: {}", e);
                        continue;
                    },
                };

                let broadcaster = &self.broadcaster.unwrap();
                let delegate = &self.delegate.unwrap();

                spawn(async move {
                    let mut buffer = vec![0; 1024];
                    'outer: loop {
                        let read = loop {
                            match stream.try_read_buf(&mut buffer) {
                                Ok(len) => {
                                    if len == 0 {
                                        break len;
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to read from stream: {}", e);
                                    break 'outer;
                                },
                            }
                        };

                        if read == 0 {
                            continue;
                        }

                        let message = match crate::protocol::deserialize(buffer[..len].to_vec()) {
                            Ok(message) => message,
                            Err(e) => {
                                println!("Failed to deserialize message: {}", e);
                                break;
                            },
                        };

                        match broadcaster.register(message.subject.as_str()).await.send(message) {
                            Ok(_) => {},
                            Err(e) => println!("Failed to send message: {}", e),
                        }
                    }
                });
            }
        });

        Ok(())
    }

    pub async fn handler(&self, message: Message) {
        
    }
}
