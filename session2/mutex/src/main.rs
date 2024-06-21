use std::sync::Mutex;
use std::thread;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
fn main() {
    let mut handlers = vec![];
    for _ in 0..10 {
        let handler = thread::spawn(|| {
            let mut lock = NUMBERS.lock().unwrap();
            let max = {
                let temp = lock.iter().max().unwrap_or(&0);
                *temp
            };
            lock.push(max + 1);
        });
        handlers.push(handler);
    }

    let lock = NUMBERS.lock().unwrap();
    println!("{:?}", lock);
}
