fn test() {
    println!("test");
}

fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // Join 2 operation
    pool.join(test, test);

    pool.spawn(|| {
        println!("Hello world!");
    });

    pool.scope(|scope| {
        for i in 0..10 {
            scope.spawn(move |_| {
                println!("{i}");
            })
        }
    });

    // Send function/job to all thread in pool for execution
    pool.broadcast(|broadcast_context| {
        println!("Hello from thread-{}", broadcast_context.index());
    });

    println!("Hello from main thread");
}
