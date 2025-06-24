// Struct 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Initialize
fn initialize_struct() {
    let user1 = User {
        email: String::from("somone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
}

fn initialize_struct_simple(email: String, username: String){
    let user1 = User {
        email,
        username,
        active: true,
        sign_in_count: 1
    };
}

// Updating
fn update_struct(){
    let mut user1 = User {
        email: String::from("somone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("someone_new@example.com");

    // Inherit the rest of the values from one user
    let user2 = User {
        email: String::from("somone@example.com"),
        username: String::from("someusername123"),
        ..user1
    };
}

// Methods
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Method (uses "&self")
    fn details(&self) -> String {
        String::from(&self.last_name)
    }

    // Associated function (does not use "&self")
    fn more_detail() -> String {
        String::from("pizza")
    }
}

fn example() {
    let george = Person {
        first_name: String::from("George"),
        last_name: String::from("Lopez"),
    };
    println!("{}", george.details());
    println!("{}", Person::more_detail());
}

fn main() {
    println!("Hello, world!");
}