extern crate countthe;

use std::io;
use std::io::Read;
use std::io::BufReader;

fn main() {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Err(error) => {
            eprintln!("error: {:?}", error);
            1
        }
        Ok(size) => size,
    };

    let mut reader = BufReader::new(buffer.as_bytes());

    println!("{}", countthe::count_the(&mut reader));
}
