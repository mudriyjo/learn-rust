// 1. Done - Trait example
// 2. Param in fn
// 3. Param from fn
// 4. Generic trait collection
// 5. As_any example
// 6. Operation overloading with Point type Output = Point;
trait Animal {
    fn say(&self);
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn say(&self) {
        println!("Meow!!!")
    }
}

impl Animal for Dog {
    fn say(&self) {
        println!("Woof!!!")
    }
}
fn main() {
    let cat = Cat;
    cat.say();
    let dog = Dog;
    dog.say();
}
