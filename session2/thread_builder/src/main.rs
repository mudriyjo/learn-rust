use std::thread;

fn my_thread() {
    println!("[THREAD-{}] Hello world!", thread::current().name().unwrap());
}
fn main() {
    println!("Hello, world!");
    thread::Builder::new()
        .name("MY-THREAD".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap();
}
