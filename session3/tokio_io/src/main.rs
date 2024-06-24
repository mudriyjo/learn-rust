use std::{io::BufRead, path::Path, time::Instant};

async fn sync_io_example() -> anyhow::Result<usize> {
    use std::fs::File;
    use std::io::BufReader;

    let now = Instant::now();
    let file = File::open(Path::new("warandpeace.txt"))?;
    let reader = BufReader::new(file);
    let mut count = 0;
    let lines = reader.lines();
    lines.for_each(|line| {
        if let Ok(line) = line {
            if !line.trim().is_empty() {
                count += 1;
            }
        }
    });

    println!(
        "Counted lines: {}, for {:.3} seconds",
        count,
        now.elapsed().as_secs_f32()
    );
    Ok(count)
}

async fn async_io_example() -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    let now = Instant::now();
    let file = File::open(Path::new("warandpeace.txt")).await?;
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            count += 1;
        }
    }

    println!(
        "Counted lines: {}, for {:.3} seconds",
        count,
        now.elapsed().as_secs_f32()
    );
    Ok(count)
}

#[tokio::main]
async fn main() {
    println!("Sync read test starting...");
    let now = Instant::now();
    let _ = tokio::join!(
        sync_io_example(),
        sync_io_example(),
        sync_io_example(),
        sync_io_example(),
    );
    println!("Sync read test finished");
    println!(
        "Total spend time is {:.3} seconds",
        now.elapsed().as_secs_f32()
    );
    println!("======================");

    println!("Async read test starting...");
    let now = Instant::now();
    let _ = tokio::join!(
        async_io_example(),
        async_io_example(),
        async_io_example(),
        async_io_example(),
    );
    println!("Sync read test finished");
    println!(
        "Total spend time is {:.3} seconds",
        now.elapsed().as_secs_f32()
    );
}
