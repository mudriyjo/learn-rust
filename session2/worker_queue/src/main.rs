use std::{
    collections::VecDeque,
    sync::{mpsc::channel, Mutex},
    thread::{self, sleep},
    time::Duration,
};

static WORKER_QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

fn main() {
    const THREAD_NUMBERS: usize = 4;
    let mut workers = Vec::with_capacity(THREAD_NUMBERS);
    let mut broadcasting_channels = Vec::with_capacity(THREAD_NUMBERS);

    for i in 0..THREAD_NUMBERS {
        let (sender, reciever) = channel::<()>();
        broadcasting_channels.push(sender);

        let handler = thread::spawn(move || {
            while let Ok(_) = reciever.recv() {
                let mut lock = WORKER_QUEUE.lock().unwrap();
                if let Some(_) = lock.pop_front() {
                    std::mem::drop(lock);
                    println!("[THREAD-{i}] Start working....");
                    thread::sleep(Duration::from_secs(2));
                    println!("[THREAD-{i}] Work is done!");
                } else {
                    println!("[THREAD-{i}] No more works...");
                }
            }
            thread::sleep(Duration::from_secs(2));
        });
        workers.push(handler);
    }

    loop {
        let signal = {
            let mut queue = WORKER_QUEUE.lock().unwrap();
            println!("Length of queue: {}", queue.len());
            if queue.len() < 5 {
                queue.push_front("Hello".to_string());
                false
            } else {
                true
            }
        };

        if signal {
            println!("Sending signals....");
            broadcasting_channels.iter().for_each(|ch| ch.send(()).unwrap());
            println!("Signals sent!");
        }
        sleep(Duration::from_secs(1));
    }
}
