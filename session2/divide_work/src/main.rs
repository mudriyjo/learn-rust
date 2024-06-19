fn main() {
    const N_THREAD: usize = 8;
    let data: Vec<u32> = (0..5000).collect();
    let data_chunk = data.chunks(N_THREAD);
    let mut thread_handler = vec![];

    for chunk in data_chunk {
        let ch = chunk.to_owned();
        thread_handler.push(std::thread::spawn(move || {
            ch.iter().sum::<u32>()
        }))
    }

    let mut sum = 0;
    for handler in thread_handler {
        sum += handler.join().unwrap();
    }

    println!("sum is: {}", sum);
}
