use std::env::args;
use std::env::current_dir;
use std::fs;
use std::path::Path;
use std::process::exit;

fn create_rit_directories(path: &str) {
    let git_root_dir = format!("{}/.git", path);
    let _ = fs::create_dir(&git_root_dir);
    let folders = ["objects", "refs"];
    for folder in folders.iter() {
        let _ = fs::create_dir(format!("{}/{}", &git_root_dir, folder));
    }
    exit(0);
}

fn create_empty_repo () {
    let args: Vec<String> = args().collect();
    if args.len() == 3 {
        let path = &args[2];
        println!("Initializing Rit repository at {}...", path);
        if !Path::new(path).exists() {
            eprintln!("error: {} does not exist ", path);
            exit(1);
        }
        create_rit_directories(path);
    } else {
        let path = current_dir().unwrap().display().to_string();
        println!("Initializing Rit repository in current directory...");
        create_rit_directories(&path);
    }
}

fn perform_command (command: &str) {
    match command {
        "help" => {
            println!("Available commands:");
            println!("  help - show this help");
            println!("  init [path] - create empty Rit repository");
        },
        "init" => {
            create_empty_repo()
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
