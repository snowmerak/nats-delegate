pub mod delegate;
pub mod callback;

pub struct MySubscriber {
}

impl callback::SubscriptionCallback for MySubscriber {
    fn on_subscription(&self, subscription: &nats::Message) {
        println!("Received message: {}", String::from_utf8_lossy(&subscription.data));
    }
}
