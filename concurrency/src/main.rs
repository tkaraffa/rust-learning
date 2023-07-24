use std::sync::mpsc;
use std::thread;
use std::time::Duration; // multiple producer, single consumer

fn main() {
    let (tx, rx) = mpsc::channel();
    // returns a tuple
    // tx: transmitter
    // rx: receiver

    // move tx into thread's closure
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));

        let val = String::from("hi");
        tx.send(val); // send it
    });

    let received = rx.try_recv(); // receive it, block main thread, wait until value sent down channel
    println!(
        "Got: {}",
        received.unwrap_or_else(|_| String::from("Error."))
    );
}
