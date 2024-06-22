use std::{sync::mpsc, thread, time::Duration};

enum Command {
    SayHello,
    Quit
}

fn main() {
    let (sx, rx) = mpsc::channel::<Command>();

    let handler = thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            thread::sleep(Duration::from_secs(1));
            match command {
                Command::SayHello => println!("Hello!"),
                Command::Quit => {
                    println!("Exiting...");
                    break
                }
            }
        }
    });

    
    for _ in 0..10 {
        println!("Send hello command...");
        sx.send(Command::SayHello).unwrap();
    }
    println!("Send exit command...");
    sx.send(Command::Quit).unwrap();
    handler.join().unwrap();
}
