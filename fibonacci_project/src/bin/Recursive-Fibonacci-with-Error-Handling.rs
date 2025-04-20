use std::io;

fn main() {
    loop {
        println!("Enter a positive number:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(n) => {
                let result = fibonacci(n);
                println!("The {}th Fibonacci number is: {}", n, result);
                break; // ✅ end the loop if input is valid
            }
            Err(_) => {
                println!("❌ Please enter a valid positive number.\n");
            }
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}