struct MyStrunc {
    num: i32
}
impl MyStrunc {
    fn new(n: i32) -> Self {
        println!("Create MyStruct: {}", n);
        MyStrunc { num:  n}
    }
}
impl Drop for MyStrunc {
    fn drop(&mut self) {
        println!("Drop MyStruc: {}", self.num);
    }
}
fn main() {
    let x = MyStrunc::new(1);
    {
        let y = MyStrunc::new(2);
    }
    println!("End of main function...");
}
