use nats_delegate::delegate;

#[tokio::main]
async fn main() {
    let mut delg = delegate::Delegate::connect("nats://localhost:4222", "", "").unwrap();

    delg.subscribe("test", Box::new(nats_delegate::MySubscriber {})).unwrap();

    delg.publish("test", &"Hello, world!".as_bytes().to_vec()).unwrap();

    delg.unsubscribe("test").await.expect("Failed to unsubscribe.");
}
