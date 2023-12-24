pub mod delegate;
pub mod callback;

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
