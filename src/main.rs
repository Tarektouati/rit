use std::env::args;
use std::process::exit;
mod cmd;

fn perform_command (command: &str) {
    match command {
        "help" => {
           cmd::help::show_help();
        },
        "init" => {
            cmd::init::create_empty_repo();
        },
        _ => {
            println!();
            eprintln!("error: unknown command {}", command);
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let command = &args[1];
    perform_command(command);
}
