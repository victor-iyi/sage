//! `sage.utils` contains useful helper functions either during using
//! `sage` or by your program (which may or may not have anything to
//! do with `sage`).
#![allow(dead_code)]

use std::io;

/// Getting input from stdin. Masking Python's `input` function.
///
/// ## Basic Usage.
/// Getting `String` input from console.
/// ```
/// let name: String = input("Enter your name:")
///                     .expect("Failed to get name.");
/// println!("name = {}", name);
/// ```
///
/// Getting other types aside `String`.
/// ```
/// let age: u8 = input("Enter your age: ")
///                .expect("Failed to get age.")
///                .parse::<u8>().expect("Invalid age.");
/// ```
pub fn input(msg: &str) -> io::Result<String> {
    use std::io::Write;
    // Print prompt to the console.
    print!("{}", msg);
    io::stdout().flush()?;

    // Accept input.
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}
