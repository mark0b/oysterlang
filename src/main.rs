#[macro_use]
extern crate lazy_static;
extern crate num_derive;

use std::io;
use std::io::Write;
use std::result;

mod ast;
mod context;
mod parser;

static PREFIX: &'static str = "🦪 Oyster Shell >";

fn main() {
    // loop {
    //     print!("{}", PREFIX);
    //     io::stdout().flush().unwrap();

    //     let mut input = String::new();
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");

    //     println!("{}", input.trim_end_matches('\n'));
    // }

    let input = "   5 + (1 + 2 + 3) - 4   ";
    let num = parser::parse(input);

    println!("|{:?}|", num)
}
