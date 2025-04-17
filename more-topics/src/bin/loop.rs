
fn main() {
    let treasures = ["Gold", "Silver", "Crystal Gem", "Emerald", "Emerald", "Emerald"];
    let mut energy = 5;

    for treasure in treasures.iter() {
        if energy == 0 {
            println!("You are out of energy. Game over!");
            break;
        }else if treasure == &"Ruby Gem" {
             println!("You found the Ruby Gem! You Win!");
             break;
        }
        energy -= 1;
    }
}