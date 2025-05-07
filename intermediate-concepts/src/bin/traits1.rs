
pub trait Vehicle {
    fn forward() -> String;
    fn backward() -> String;
    fn turn_ignition() -> String {
        String::from("vroom vroom")
    }
}

pub struct Car {
    color: String,
}

impl Vehicle for Car {
    // without &self = associated fun
    fn forward() -> String {
        String::from("Driving my car forward")
    }
    fn backward() -> String {
        "Backing my car up...".to_string()
    }
}

pub struct Truck {
    color: String
}

impl Vehicle for Truck {
    fn forward() -> String {
        "Driving my truck forward".to_string()
    }

    fn backward() -> String {
        "Backing my truck up...".to_string()
    }
}

fn main() {
    println!("{}", Car::turn_ignition());
    println!("{}", Car::forward());
    println!("{}", Car::backward());
    println!("{}", Truck::turn_ignition());
    println!("{}", Truck::forward());
    println!("{}", Truck::backward());
}