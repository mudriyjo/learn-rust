struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purrrr...)", self.0);
    }
}
struct CatFeeder<'a>{
    cat: &'a mut Cat
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut cats = vec![
        Cat("Jhony".to_string()),
        Cat("Bobby".to_string())
    ];

    let mut feeder = vec![];

    for cat in cats.iter_mut() {
        feeder.push(cat);
    }

    feeder.iter_mut().for_each(|f| f.feed());
    cats.iter().for_each(|c| println!("{:?}", c.0))
}
