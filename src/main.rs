use std::process::{Command, Stdio};
use std::io::Read;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: clip <command> [<args>...]");
        std::process::exit(1);
    }

    let command = &args[1];
    let command_args = &args[2..];

    let output = Command::new(command)
        .args(command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut output_str = String::new();
    output_str.push_str(&String::from_utf8_lossy(&output.stdout));
    output_str.push_str(&String::from_utf8_lossy(&output.stderr));

    clipboard.set_contents(output_str.clone()).unwrap();

    println!("{}", output_str);
}
