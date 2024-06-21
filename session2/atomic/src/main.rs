use std::sync::atomic::AtomicI64;
use std::thread;

static COUNTER: AtomicI64 = AtomicI64::new(0);
fn main() {
    let mut handlers = vec![];
    for _ in 0..1000 {
        handlers.push(thread::spawn(|| {
            for _ in 0..1000 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }))
    }
    handlers.into_iter().for_each(|handler| handler.join().unwrap());
    
    println!("Sum: {}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
