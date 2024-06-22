use std::{sync::mpsc::channel, thread, time::Duration};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit
}
fn say_hello() {
    println!("Say hello!");
}

fn main() {
    let (sx, rx) = channel::<Command>();
    let handler = thread::spawn(move || {
        println!("Worker initialize...");
        thread::sleep(Duration::from_secs(2));
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break
            }
        }
    });

    println!("Send fn to worker...");
    sx.send(Command::Run(Box::new(|| println!("Custom closure print....")))).unwrap();
    sx.send(Command::Run(Box::new(say_hello))).unwrap();

    println!("Send stop work to worker...");
    sx.send(Command::Quit).unwrap();

    println!("Join thread...");
    handler.join().unwrap();
}
