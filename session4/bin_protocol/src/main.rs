use std::{error::Error, fs::File, io::Write};

use bytemuck::{Pod, Zeroable};

// 1. Done - Implement bin encoding and save to file using bytemuck
// 2. Implement bin protocol save to file and read 
//    it from file by order (separate struct fields) (using le_bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
struct MyStruct {
    num: u32,
    data: [u8; 8],
}

fn bytemuck_example() -> Result<(), Box<dyn Error>> {
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

struct MyNewStruct {
    pub num: u32,
    pub data: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // bytemuck_example();

    let my_new_struct = MyNewStruct {
        num: 10,
        data: "Hello world".to_string()
    };

    let mut file = File::create("./bin_my_struct.bin")?;
    let num_bytes = my_new_struct.num.to_le_bytes();
    let str_len = my_new_struct.data.len().to_le_bytes();
    let str_data_bytes = my_new_struct.data.as_bytes();

    file.write_all(&num_bytes)?;
    file.write_all(&str_len)?;
    file.write_all(str_data_bytes)?;

    let file_bytes = std::fs::read("./bin_my_struct.bin")?;
    let num = u32::from_le_bytes(file_bytes[0..4].try_into()?);
    let str_len = usize::from_le_bytes(file_bytes[4..12].try_into()?);
    let str_data = String::from_utf8(file_bytes[12..(12 + str_len)].to_vec())?;

    println!("Result num: {}, data: {}", num, str_data);
    Ok(())
}
