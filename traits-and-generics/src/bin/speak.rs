trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;


impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn make_it_talk<T: Speak>(animal: T) {
    animal.speak();
}



fn main() {
    let dog = Dog;
    let cat = Cat;

    make_it_talk(dog);
    make_it_talk(cat);
}