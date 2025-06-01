use std::io;
use std::io::Write;

// U can customize the promt however you want
pub fn draw_prompt() {
    let current_path = std::env::current_dir().unwrap();

    print!("\x1b[32m{}\x1b[31m -> \x1b[0m", current_path.display());
    _ = io::stdout().flush();
}
