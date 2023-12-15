use std::io::Write;

pub fn clear() {
    print!("\x1b[H\x1b[2J");
    std::io::stdout().flush().unwrap();
}
