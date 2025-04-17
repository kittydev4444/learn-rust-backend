fn main(){
    let mut items = vec!["Gold", "Silver", "Ruby Gem", "Emerald"];

    items.push("Diamond");

    items.remove(1);

    println!("Items : {:?}", items);
    println!("Items length: {}", items.len());
    println!("Items capacity: {}", items.capacity());
}