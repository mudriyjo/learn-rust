use std::error::Error;

use bytemuck::{Pod, Zeroable};

// 1. Implement bin encoding and save to file using bytemuck
// 2. Implement bin protocol save to file and read 
//    it from file by order (separate struct fields) (using le_bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
struct MyStruct {
    num: u32,
    data: [u8; 8],
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = vec![
        MyStruct{
            num: 10,
            data: b"Hello   ".to_owned(),
        },
        MyStruct{
            num: 10,
            data: b"World!  ".to_owned(),
        }
    ];

    let bytes: &[u8] = bytemuck::cast_slice(&data);
    std::fs::write("./data.bin", bytes)?;

    let read = std::fs::read("./data.bin")?;
    let my_struct: &[MyStruct] = bytemuck::cast_slice(&read);
    println!("Readed structure {:?}", my_struct);
    Ok(())
}
