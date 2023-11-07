use core::time;
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep, JoinHandle},
};

use channels::mpsc::channel;

fn main() {
    let (mut sender, receiver) = channel::<usize>();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers: Vec<JoinHandle<()>> = vec![];

    for id in 0..4 {
        let receiver = Arc::clone(&receiver);
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            println!("worker {} receives: {:?}", id, message);
            sleep(time::Duration::from_secs(2));
        });
        workers.push(thread);
    }

    for message in 0..10 {
        sender.send(message);
    }

    for worker in workers {
        worker.join().unwrap();
    }
}
