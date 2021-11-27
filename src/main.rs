use std::env::args;
use std::env::current_dir;
use std::fs;


fn create_empty_repo () {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let path = current_dir().unwrap().display().to_string();
        println!("Initializing git repository in current directory...");
        let git_root_dir = format!("{}/.git", path);
        let _ = fs::create_dir(&git_root_dir);
        let folders = ["objects", "refs"];
        for folder in folders.iter() {
            let _ = fs::create_dir(format!("{}/{}", &git_root_dir, folder));
        }

    } else {
        println!("TODO : Initializing git repository at {}...", args[2]);
    } 
}

fn perform_command (command: &str) {
    match command {
        "help" => {
            println!("Available commands:");
            println!("  help - show this help");
            println!("  init - run the program");
        },
        "init" => {
            create_empty_repo()
        },
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let command = &args[1];
    perform_command(command);
}
