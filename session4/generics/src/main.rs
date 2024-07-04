fn just_print<T>(x: T)
where
    T: ToString,
{
    println!("{}", x.to_string())
}

fn main() {
    just_print(1);
    just_print("Hello");
}
