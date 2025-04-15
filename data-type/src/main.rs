fn main() {
    let x = 5;
    let y = 0.5;

    let z = x + y as i32;
    
    let msg = String::from("Hello, world!");
    let msg1 = "Hello, world!".to_string();
    let msg2 = "Hello, world!";
    let msg4 = format!("Point: ({}, {})", x, y);

    println!("x: {}, y: {}, z: {}", x, y, z);
}
