use nats::Message;


pub trait SubscriptionCallback where Self: Send + Sync {
    fn on_subscription(&self, subscription: &Message, is_request: bool) -> Option<Vec<u8>>;
}