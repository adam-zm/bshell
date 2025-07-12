use std::env;
use std::io::{self, Write, stdout};
use std::process::Command;

fn main() {
    let mut buffer: String = String::new();
    let home_dir = env::home_dir().unwrap();

    loop {
        let current_dir = env::current_dir().unwrap();

        stdout().flush().unwrap();
        print!(
            "{} -> ",
            current_dir.clone().into_os_string().into_string().unwrap()
        );
        stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        let mut parts = buffer.split_whitespace();
        let command = parts.next().unwrap();

        if command == "exit" {
            println!("Bye!");
            std::process::exit(0);
        } else if command == "cd" {
            let mut path: String = parts.clone().collect();
            if path.len() < 1 {
                path = home_dir.clone().into_os_string().into_string().unwrap();
            }
            match env::set_current_dir(path) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            }
            buffer.clear();
            continue;
        }

        let process = Command::new(command).args(parts).spawn();
        match process {
            Ok(mut child) => {
                child.wait().expect("Error");
            }
            Err(err) => {
                println!("{}", err);
            }
        }
        buffer.clear();
    }
}
