// 1. Done - Trait example
// 2. Done - Param in fn
// 3. Done - Param from fn
// 4. Done - Generic trait collection
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

fn say_twice(animal: &impl Animal) {
    animal.say();
    animal.say();
}

fn create_animal() -> impl Animal {
    Cat
}

fn main() {
    let cat = Cat;
    cat.say();
    let dog = Dog;
    dog.say();

    say_twice(&dog);
    let animal = create_animal();
    say_twice(&animal);

    println!("\nCollection of animals:");
    // We need Box and dyn cause we can't calculate size of trait in compile time
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];
    animals.iter().for_each(|a| a.say());
}
