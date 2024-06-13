fn greet(name: String) {
    println!("Hello, {name}!");
}

fn greet_with_return(name: String) -> String {
    println!("Hello, {name}!");
    name
}

fn greet_with_borrowing(name: &mut String) {
    *name = format!("Mr. {}!", name);
    println!("Hello, {name}");
}
fn main() {
    let num: i32 = 5;
    println!("{num}");
    
    // mutation
    let mut num2: i32 = 6;
    println!("{num2}");
    num2 = 7;
    println!("{num2}");

    let mut name: String = "Alex".to_string();
    // Option 1
    // greet(name);
    // Doesn't work because previous was borrowed
    //greet(name);

    // Option 2
    // Clone original string
    greet(name.clone());
    greet(name.clone());

    // Option 3
    // Copy whole string
    let mut name = greet_with_return(name);
    let mut name = greet_with_return(name);

    // Option 4
    greet_with_borrowing(&mut name);
    println!("Updated name is: {name}");
}
