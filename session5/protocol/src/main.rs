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
fn main() {
    println!("Hello, world!");
}
