trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof! My name is {}", self.name);
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow~ I'm called {}", self.name);
    }
}

fn make_it_talk<T: Speak>(animal: T) {
    animal.speak();
}
fn main() {
    let dog = Dog{ name: String::from("Max")};
    let cat = Cat{name: String::from("Luna")};

    make_it_talk(dog);
    make_it_talk(cat);
}