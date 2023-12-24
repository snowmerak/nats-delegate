use nats_delegate::{delegate, callback};

#[tokio::main]
async fn main() {
    let mut delg = delegate::Delegate::connect("nats://localhost:4222", "", "").unwrap();

    delg.subscribe("test", Box::new(nats_delegate::MySubscriber {})).unwrap();

    delg.publish("test", &"Hello, world!".as_bytes().to_vec()).unwrap();

    delg.unsubscribe("test").await.expect("Failed to unsubscribe.");
}

pub struct MySubscriber {
}

impl callback::SubscriptionCallback for MySubscriber {
    fn on_subscription(&self, subscription: &nats::Message, is_request: bool) -> Option<Vec<u8>> {
        println!("Received message: {}", String::from_utf8_lossy(&subscription.data));

        if is_request {
            Some("Hello, world!".as_bytes().to_vec())
        } else {
            None
        }
    }
}