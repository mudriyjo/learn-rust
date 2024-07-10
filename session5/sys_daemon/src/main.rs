use std::{
    collections::VecDeque,
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

fn send(tcp_stream: &mut TcpStream, bytes: &Vec<u8>) -> anyhow::Result<()> {
    tcp_stream.write_all(bytes)?;
    Ok(())
}

fn send_many_command(tcp_stream: &mut TcpStream, commands: &mut VecDeque<Vec<u8>>) {
    while let Some(message) = commands.pop_front() {
        tracing::info!("bytes send: {}", message.len());

        if let Err(e) = tcp_stream.write(&message) {
            commands.push_front(message);
            tracing::error!("Can't send other messages cause error: {}", e);
            return;
        }
    }
}

fn send_command(reciever: &Receiver<CollectorCommand>, queue: &mut VecDeque<Vec<u8>>) {
    if let Ok(command) = reciever.recv() {
        let bytes = protocol::encode_v1(command);
        queue.push_front(bytes);
        println!("queue len: {}", queue.len());

        if let Ok(mut tcp_stream) = TcpStream::connect(DAEMON_COLLECTOR_ADDRESS) {    
            if queue.len() > 1 {
                send_many_command(&mut tcp_stream, queue);
            } else {
                let message = queue
                    .pop_front()
                    .expect("Non empty queue for some reason is empty...");

                tracing::info!("bytes send: {}", message.len());

                if let Err(e) = send(&mut tcp_stream, &message) {
                    tracing::error!("Can't write to the buffer 2048 Bytes size, error: {}", e);
                    queue.push_front(message);
                }
            }
        } else {
            tracing::error!("Connection refused. Trying reconnect");
            thread::sleep(Duration::from_secs(1));
        }
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
// 1. Add custom errors type using thiserror crate to handle problem with connections
fn main() -> anyhow::Result<()> {
    color_eyre::install().expect("Error with starting color eyre hook...");

    tracing_subscriber::fmt::init();

    let collector_id = init_id_collector()?;
    let (sender, reciever) = mpsc::channel::<CollectorCommand>();

    std::thread::spawn(move || {
        gathering_info(collector_id, sender);
    });

    let mut queue: VecDeque<Vec<u8>> = VecDeque::with_capacity(100);
    loop {
        send_command(&reciever, &mut queue);
    }
}
