use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

fn producer(producer_id: u8, tx: Sender<String>) {
    let mut messages = vec![];
    for x in 0..3 {
        messages.push(format!("producer {}: message: {}", producer_id, x))
    }
    for message in messages {
        println!("{}", message);
        tx.send(message).unwrap();
    }
}

fn consumer(consumer_id: u8, rx: Receiver<String>){
    for received in rx {
        println!("Got: {} on consumer: {}", received, consumer_id);
    }
    println!("finished receiving !");
}

fn main() {
    const NOF_PRODUCERS: u8 = 3;

    let (tx, rx) = mpsc::channel();
    let mut thread_handles = vec![];
    println!("Spawning producers");
    for i in 0..NOF_PRODUCERS {
        let new_tx = tx.clone();
        let handle = thread::spawn( move || {
            producer(i, new_tx)
        });
        thread_handles.push(handle);
    }

    let handle = thread::spawn( move || {
        consumer(0, rx)
    });

    println!("starting to join threads !");

    for thread_handle in thread_handles{
        thread_handle.join().unwrap();
    }
    println!("dropping the last tx");
    drop(tx);
    handle.join().unwrap();

}
