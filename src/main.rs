mod prompt;
use std::io;
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    loop {
        prompt::draw_prompt();

        // handling input from user
        // TODO: implement | and >>
        let mut input = String::new();
        _ = io::stdin().read_line(&mut input);
        let mut parts = input.trim().split_whitespace();
        let command = parts.next();
        let command_safe: &str;

        match command {
            None => {
                println!("");
                continue;
            }
            Some(command_str) => {
                command_safe = command_str;
            }
        };

        let args = parts;

        match command_safe {
            "exit" => {
                print!("Bye!\n");
                break;
            }
            "cd" => {
                let path = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(path);
                if let Err(e) = std::env::set_current_dir(&root) {
                    println!("{}", e);
                }
            }
            _ => {
                let mut process = Command::new(command_safe).args(args).spawn();

                match &mut process {
                    Ok(process) => {
                        _ = process.wait();
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
