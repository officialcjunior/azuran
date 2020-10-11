use std::env;
use std::process::Command;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let _file_name = &_args[1];
    let output = Command::new("g++")
        .arg("-Wfatal-errors")
        .arg("-w")
        .arg(_file_name)
        .output()
        .expect("Failed");
    println!("Compilation status:");

    if !output.stderr.is_empty() {
        println!(
            "\x1b[0;31mstderr: {}\x1b[0m",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        println!("Compilation successful");
    }
}
