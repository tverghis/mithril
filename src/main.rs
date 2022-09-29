#![warn(clippy::pedantic)]

mod demo;
mod error;

use demo::Demo;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let demo = Demo::try_from_read(&mut stdin).unwrap();

    println!("{:#01x?}", &demo.bytes()[0..32]);
}
