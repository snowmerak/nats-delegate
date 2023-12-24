use nats_delegate::delegate;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    let mut delg = delegate::Delegate::connect("nats://localhost:4222", "", "").unwrap();

    delg.subscribe("test", Box::new(nats_delegate::MySubscriber {})).unwrap();

    let (tx, mut rx) = channel(1);

    ctrlc::set_handler(move || {
        tx.try_send(()).unwrap();
    }).unwrap();

    println!("Press Ctrl-C to exit.");
    rx.recv().await.unwrap();

    delg.unsubscribe("test").await.expect("Failed to unsubscribe.");
    println!("Exiting.")
}