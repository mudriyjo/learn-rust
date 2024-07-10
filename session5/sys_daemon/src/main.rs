use std::{
    io::Write,
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use protocol::CollectorCommand;
use sysinfo::System;

const DAEMON_COLLECTOR_ADDRESS: &str = "0.0.0.0:9444";

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
            tracing::error!("Get error when try to send command: {:?}", &e.0);
        }

        let since_start = now.elapsed().as_secs_f32();
        if since_start < 1.0 {
            thread::sleep(Duration::from_secs_f32(1.0 - since_start));
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn send_command(reciever: &Receiver<CollectorCommand>) {
    if let Ok(mut tcp_stream) = TcpStream::connect(DAEMON_COLLECTOR_ADDRESS) {
        if let Ok(command) = reciever.recv() {
            let bytes = protocol::encode_v1(command);

            tracing::info!("bytes send: {}", bytes.len());

            if let Err(e) = tcp_stream.write_all(&bytes) {
                tracing::error!("Can't write to the buffer 2048 Bytes size, error: {}", e)
            }
        }
    } else {
        tracing::error!("Connection refused. Trying reconnect");
        thread::sleep(Duration::from_secs(1));
    }
}

fn init_id_collector() -> anyhow::Result<u128> {
    let path = std::path::Path::new("./uuid");
    if path.exists() {
        let res = std::fs::read_to_string(path)?;
        let uuid = res.parse::<u128>()?;
        Ok(uuid)
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string())?;
        Ok(uuid)
    }
}

// TODO
// 2. Add DeQueue for 100 record to hold them while server is not working
// 3. Add method send queue to send whole queue in 1 single tcp connection
// 4. Add custom errors type using thiserror crate to handle problem with connections
fn main() -> anyhow::Result<()> {
    color_eyre::install().expect("Error with starting color eyre hook...");

    tracing_subscriber::fmt::init();

    let collector_id = init_id_collector()?;
    let (sender, reciever) = mpsc::channel::<CollectorCommand>();

    std::thread::spawn(move || {
        gathering_info(collector_id, sender);
    });

    loop {
        send_command(&reciever);
    }
}
