use std::{io::{Error, ErrorKind}, process::Command};

fn error_handling_example(dir: &str) -> Result<i32, Error> {
    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    list_cmd.current_dir(dir).status()?;

    println!("\n\n");

    Ok(1)
}

fn main() {
    error_handling_example("src");
    error_handling_example("lib");
}
