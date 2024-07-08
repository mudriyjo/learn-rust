use std::{
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

use protocol::CollectorCommand;
use sysinfo::System;

fn gathering_info(collector_id: u128, tx: Sender<protocol::CollectorCommand>) {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();
        let now = Instant::now();

        let total_memory = sys.available_memory();
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

fn main() {
    println!("Hello, world!");
}
