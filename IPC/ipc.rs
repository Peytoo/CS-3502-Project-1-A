use std::process::{Command, Stdio};
use std::io::{self, Read};

fn main() {
    // Spawn the `ls` command and capture its output
    let output = Command::new("ls")
        .stdout(Stdio::piped()) // Redirect stdout to a pipe
        .spawn()
        .expect("Failed to execute ls command");

    // Read from the pipe
    if let Some(mut stdout) = output.stdout {
        let mut buffer = String::new();
        stdout.read_to_string(&mut buffer).expect("Failed to read stdout");

        // Print the captured output
        println!("{}", buffer);
    }
}
