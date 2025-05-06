use std::{io::ErrorKind, process::Command};

fn os_commands_example() {
    let echo_cmd = if cfg!(target_or = "windows") {
        Command::new("cmd")
                            .args(["/C", "echo aye there from Windows!"])
                            .output()
                            .expect("Failed to execute command.")
    } else {
        Command::new("sh")
        .args(["-c", "echo ahoy there from Linux!"])
        .output()
        .expect("Failed to execute command.")
    };
    println!("\n\n");
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte response.");
    println!("{}", cmd_output);
    println!("\n\n");
}

fn os_commands_example_2() {
    println!("\n\n");

    let mut cmd_root = Command::new("ls");

    cmd_root.status().expect("Failed to execute list command.");

    println!("\n\n");

    cmd_root.current_dir("src").status().expect("Failed to execute list command.");

    println!("\n\n");
}

fn error_handling_example(dir: &str) {
    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    match list_cmd.current_dir(dir).status() {
        Ok(cmd) => Some(cmd),
        Err(e) =>  match e.kind() {
            ErrorKind::NotFound => {
                println!("Directory not found!");
                None
            },
            _ => panic!("An unexpected error has occured!")
        }
    };

    println!("\n\n");
}

fn main() {
    os_commands_example();
    os_commands_example_2();
    error_handling_example("src");
    error_handling_example("lib");
}
