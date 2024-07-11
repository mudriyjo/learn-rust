use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/*
Bytes        | Name           | Description
===========================================================================================
0 - 1        | Magic Number   | Sending a magic number to ensure that it us what you expect
2 - 3        | Version Number | Version number from 1 to 65,535
4 - 7        | Timestamp      | 32-bit timestamp 1970-01-01 to 2106-02-07
8 - 11       | Message size   | 32-but integer to represent size of message
12 - 15      | Payload size   | 32-but integer to represent size of payload
15 +         | Paylod         | Start from JSON and move to something efficient
End-4 to End | CRC32          | CRC32 checksum
===========================================================================================
*/
const MAGIC_NUMBER: u16 = 2222;
const VERSION_NUMBER: u16 = 1;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CollectorCommand {
    SubmitData {
        collector_id: u128,
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
}

#[derive(Debug)]
pub enum Commands {
    Command((u32, CollectorCommand)),
    Commands(Vec<(u32, CollectorCommand)>),
}

pub fn encode_v1(command: CollectorCommand) -> Vec<u8> {
    let payload_str = serde_json::to_string(&command).expect("Can't deserialize Collector command");
    let payload_bytes = payload_str.as_bytes();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Problem with getting timestmap")
        .as_secs() as u32;
    let timestamp_bytes = timestamp.to_be_bytes();

    let payload_size = (payload_bytes.len() as u32).to_be_bytes();

    let crc32 = crc32fast::hash(payload_bytes).to_be_bytes();

    let message_size: u32 = (MAGIC_NUMBER.to_be_bytes().len()
        + VERSION_NUMBER.to_be_bytes().len()
        + timestamp_bytes.len()
        + 4 // for message size
        + payload_size.len()
        + payload_bytes.len()
        + crc32.len()) as u32;
    let mut result = Vec::with_capacity(message_size as usize);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp_bytes);
    result.extend_from_slice(&message_size.to_be_bytes());
    result.extend_from_slice(&payload_size);
    result.extend_from_slice(payload_bytes);
    result.extend_from_slice(&crc32);
    result
}

pub fn decode_v1(bytes: &[u8]) -> Commands {
    let mut result: Vec<&[u8]> = Vec::new();
    let mut iterative_bytes = bytes;
    while !iterative_bytes.is_empty() {
        let message_size = u32::from_be_bytes([
            iterative_bytes[8],
            iterative_bytes[9],
            iterative_bytes[10],
            iterative_bytes[11],
        ]);
        result.push(&iterative_bytes[0..(message_size as usize)]);
        if message_size as usize >= iterative_bytes.len() {
            iterative_bytes = &[];
        } else {
            iterative_bytes = &iterative_bytes[(message_size as usize)..];
        }
    }
    if result.len() == 1 {
        Commands::Command(decode_helper(bytes))
    } else {
        Commands::Commands(result.into_iter().map(decode_helper).collect())
    }
}

pub fn decode_helper(bytes: &[u8]) -> (u32, CollectorCommand) {
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number: u16 = u16::from_be_bytes([bytes[2], bytes[3]]);

    assert_eq!(magic_number, MAGIC_NUMBER);
    assert_eq!(version_number, VERSION_NUMBER);

    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    // let message_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    let payload_size = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
    let crc32_transfered = u32::from_be_bytes([
        bytes[16 + payload_size as usize],
        bytes[17 + payload_size as usize],
        bytes[18 + payload_size as usize],
        bytes[19 + payload_size as usize],
    ]);

    let crc32_calculated = crc32fast::hash(&bytes[16..(16 + payload_size as usize)]);

    assert_eq!(crc32_calculated, crc32_transfered);

    let payload: CollectorCommand =
        serde_json::from_slice(&bytes[16..(16 + payload_size as usize)])
            .expect("Error decoding bytes to CollectorCommand");
    (timestamp, payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_one_and_decoding() {
        let command = CollectorCommand::SubmitData {
            collector_id: 1,
            total_memory: 1024,
            used_memory: 512,
            average_cpu_usage: 50.2,
        };

        let encoded_msg = encode_v1(command.clone());

        if let Commands::Command((_timestamp, decoded_command)) = decode_v1(&encoded_msg) {
            assert_eq!(command, decoded_command);
        } else {
            unreachable!("Should decode one command to one Commands...");
        }
    }

    #[test]
    fn test_encoding_many_and_decoding() {
        let commands = vec![CollectorCommand::SubmitData {
            collector_id: 1,
            total_memory: 1024,
            used_memory: 512,
            average_cpu_usage: 50.2,
        },
        CollectorCommand::SubmitData {
            collector_id: 2,
            total_memory: 1024,
            used_memory: 512,
            average_cpu_usage: 50.2,
        }];

        let encoded_msg: Vec<u8> = commands.clone().into_iter().map(encode_v1).flatten().collect();

        if let Commands::Commands(command_list) = decode_v1(&encoded_msg) {
            let result_commands: Vec<CollectorCommand> = command_list.into_iter().map(|el| el.1).collect();
            assert_eq!(commands, result_commands);
        } else {
            unreachable!("Should decode one command to one Commands...");
        }
    }
}
