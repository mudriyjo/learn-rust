use std::thread;
fn main() {
    const N_THREAD: usize = 8;
    let data: Vec<u32> = (0..5000).collect();
    let sum = thread::scope(|scope| {
        let chunks: Vec<&[u32]> = data.chunks(N_THREAD).collect();
        let mut thread_handlers = vec![];
        for chunk in chunks {
            let handler = scope.spawn(|| chunk.iter().sum::<u32>());
            thread_handlers.push(handler);
        }
        thread_handlers
            .into_iter()
            .map(|handler| handler.join().unwrap())
            .sum::<u32>()
    });

    println!("Sum: {sum}");
}
