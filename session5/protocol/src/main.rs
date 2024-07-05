/*
Bytes        | Name           | Description
===========================================================================================
0 - 1        | Magic Number   | Sending a magic number to ensure that it us what you expect
2 - 3        | Version Number | Version number from 1 to 65,535
4 - 7        | Timestamp      | 32-bit timestamp 1970-01-01 to 2106-02-07
8 - 11       | Payload size   | 32-but integer to represent size of payload
12 +         | Paylod         | Start from JSON and move to something efficient
End-4 to End | CRC32          | CRC32 checksum
===========================================================================================
*/
const MAGIC_NUMBER: u8 = 2222;
const VERSION_NUMBER: u16 = 1;
enum CollectorCommand {
    SubmitData {
        collector_id: u128,
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    }
}
fn main() {
    println!("Hello, world!");
}

pub fn encode_v1(command: CollectorCommand) -> (u32, Vec<u8>) {
    vec![]
}

pub fn decode_v1(butes: Vec<u8>) -> CollectorCommand {
    
}