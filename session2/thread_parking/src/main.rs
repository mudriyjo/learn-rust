use std::{io::Write, thread};

fn parking_thread(n: u32) {
    loop {
        thread::park();
        println!("Thread {n} was unparked...")
    }
}

fn read_line() -> String {
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).unwrap();
    res.trim().to_string()
}
fn main() {
    let mut handlers = vec![];
    for i in 0..10 {
        let handler = thread::spawn(move || {
            parking_thread(i);
        });
        handlers.push(handler);
    }

    loop {
        println!("Write number of thread to unpark (q for exit) :");
        std::io::stdout().flush();
        let input = read_line();
        if input == "q" {
            break;
        } else {
            let num = input.parse::<usize>();
            if let Ok(n) = num {
                if n > 0 && n < 10 {
                    handlers[n].thread().unpark();
                }
            }
        }
    }
}
