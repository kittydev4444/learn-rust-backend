

fn main(){
    let map = String::from("Old map");

    let borrowed_mab = map.as_str();

    let mut crabby_mab = borrowed_mab.to_string();

    crabby_mab.push_str(" to new map");

    println!("crabby_map: {}", crabby_mab);
}