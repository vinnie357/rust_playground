extern crate ferris_says;

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

// fn main() {
//     println!("Hello, world!");
// }

fn main() {
    let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    let message = String::from("Hello, Declan!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
