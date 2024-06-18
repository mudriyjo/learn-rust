fn print_thread() {
    println!("Hello from thread");
}

fn one_thread() {
    let thread_handle = std::thread::spawn(print_thread);
    println!("Hello from main thread!");
    thread_handle.join().unwrap();
}
fn do_math(n: u32, _vec: Vec<u32>) -> u32 {
    let mut res = n + 1;
    for i in 0..10 {
        res *= 2;
    }
    res
}

fn multiply_threads() {
    let mut handler_threads = vec![];
    for i in 0..=10 {
        let test: Vec<u32> = vec![];
        let thread_handle = std::thread::spawn(move || do_math(i,test));
        handler_threads.push(thread_handle);
        // Vector already borrowed
        // println!("{:?}", test)
    }

    handler_threads
    .into_iter()
    .for_each(|h| {
        let res = h.join().unwrap();
        println!("{}", res);
    })
}

fn main() {
    // one_thread();
    multiply_threads();
}
