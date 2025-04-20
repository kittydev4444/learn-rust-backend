use std::io::{self, Read};

fn main() {
    println!("Enter a number");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

    let n: i32 = input.trim().parse().expect("Please type a number!");

    let result = fibonacci(n);

    println!("The {}th Fibonacci number is: {}", n, result);
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
       return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}