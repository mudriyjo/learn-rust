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

fn my_own(my_struct: MyStrunc) {
}

fn my_borrow(my_struct: &MyStrunc) {
}
fn main() {
    let x = MyStrunc::new(1);
    let z = MyStrunc::new(3);
    let z2 = MyStrunc::new(4);
    {
        let y = MyStrunc::new(2);
    }
    my_own(z);
    println!("Function own finish here...");

    my_borrow(&z2);
    println!("Function borrow finish here...");
    
    println!("End of main function...");
}
