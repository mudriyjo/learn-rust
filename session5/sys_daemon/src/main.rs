use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Sender},
    thread,
    time::{Duration, Instant},
};

use protocol::CollectorCommand;
use sysinfo::System;

const DAEMON_COLLECTOR_ADDRESS: &'static str = "0.0.0.0:9444";

fn gathering_info(collector_id: u128, tx: Sender<protocol::CollectorCommand>) {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();
        let now = Instant::now();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let cpu_count = sys.cpus().len();
        let cpu_total_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
        let average_cpu_usage: f32 = cpu_total_usage / cpu_count as f32;

        let command = CollectorCommand::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        };

        if let Err(e) = tx.send(command) {
            println!("Get error when try to send command: {:?}", &e.0);
        }

        let since_start = now.elapsed().as_secs_f32();
        if since_start < 1.0 {
            thread::sleep(Duration::from_secs_f32(1.0 - since_start));
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}

// TODO
// 1. Change println to tracer
// 2. Move send to function
// 3. Change hardcoded collector id to getting it from env
fn main() {
    let (sender, reciever) = mpsc::channel::<CollectorCommand>();

    std::thread::spawn(move || {
        gathering_info(1, sender);
    });

    loop {
        let mut tcp_stream = TcpStream::connect(DAEMON_COLLECTOR_ADDRESS).unwrap();
        if let Ok(command) = reciever.recv() {
            let bytes = protocol::encode_v1(command);
            if let Err(e) = tcp_stream.write_all(&bytes) {
                println!("Can't write to the buffer 2048 Bytes size, error: {}", e)
            }
        }
    }
}
