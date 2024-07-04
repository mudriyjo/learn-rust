struct OneByte {
    n: u8
}
struct TwoByte {
    n: u16
}

#[repr(C)]
struct ThreeByte {
    x: u16,
    n: u8
}
struct FourByte {
    n: u32
}
fn main() {
    println!("{}", std::mem::size_of::<OneByte>());
    println!("{}", std::mem::size_of::<TwoByte>());
    println!("{}", std::mem::size_of::<ThreeByte>());
    println!("{}", std::mem::size_of::<FourByte>());
}
