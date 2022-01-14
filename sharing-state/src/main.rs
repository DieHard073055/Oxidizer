use std::sync::{Arc, Mutex};
use std::thread;

fn obtain_lock_and_change_the_value_using_mutex() {
    let mutex_m = Mutex::new(5);
    {
        let mut num = mutex_m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", mutex_m);
}

fn count_mutexed_variable() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", counter);
}
fn main() {
    obtain_lock_and_change_the_value_using_mutex();
    count_mutexed_variable();
}
