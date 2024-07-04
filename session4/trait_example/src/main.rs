// 1. Done - Trait example
// 2. Done - Param in fn
// 3. Done - Param from fn
// 4. Done - Generic trait collection
// 5. Done - As_any example
// 6. Operation overloading with Point type Output = Point;

use std::any::Any;

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

trait AnimalWithSize: Sized {
    fn say(&self);
}
enum SizedAnimals {
    SizedCat,
    SizedDog,
}

impl AnimalWithSize for SizedAnimals {
    fn say(&self) {
        match &self {
            Self::SizedCat => println!("MMMMMEOW"),
            Self::SizedDog => println!("WOOOOEFFF"),
        }
    }
}

trait DowncastingAnimal: Any {
    fn say(&self);
    fn as_any(&self) -> &dyn Any;
}

struct DowncatingCat;
struct DowncatingDog;

impl DowncastingAnimal for DowncatingCat {
    fn say(&self) {
        println!("meow meow moew")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl DowncastingAnimal for DowncatingDog {
    fn say(&self) {
        println!("bark bark bark")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
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

    let sized_cat = SizedAnimals::SizedCat;
    let sized_dog = SizedAnimals::SizedDog;
    let sized_animals: Vec<SizedAnimals> = vec![sized_cat, sized_dog];
    sized_animals.iter().for_each(|a| a.say());

    // Downcasting example
    let downcasting_cat = DowncatingCat;
    let downcasting_dog = DowncatingDog;
    let downcasting_animals: Vec<&dyn Any> = vec![downcasting_cat.as_any(), downcasting_dog.as_any()];
    downcasting_animals.iter().for_each(|a| {
        if let Some(animal) = a.downcast_ref::<DowncatingDog>() {
            println!("This is dog");
            animal.say();
        }
        if let Some(animal) = a.downcast_ref::<DowncatingCat>() {
            println!("This is cat");
            animal.say();
        }
    });
}
