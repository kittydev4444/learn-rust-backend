fn main() {
    let mut treasure = String::from("gold coins");

    let luffy = &treasure; // luffy borrows treasure
    let zoro = &treasure; // zoro borrows treasure
    // luffy and zoro can read treasure, but cannot modify it
    println!("Luffy sees: {}", luffy);
    println!("Zoro sees: {}", zoro);


    let trusted_friend = &mut treasure; // trusted_friend borrows treasure mutably
    // trusted_friend can modify treasure, but cannot be borrowed again until it is returned
    trusted_friend.push_str(" and jewels");
    println!("Trusted friend sees: {}", trusted_friend);
}
