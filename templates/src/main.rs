// Rust edition: 2024

use std::io::{self, BufRead as _, Write as _};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t = buf.trim().parse().unwrap();
    for _ in 0u32..t {
        writeln!(stdout, "Hello, world!").unwrap();
    }
}
