#![feature(box_syntax, box_patterns, half_open_range_patterns, or_patterns)]

#[macro_use]
extern crate lazy_static;
use std::io::Write;
use std::{env, io};

mod interpreter;
mod parser;
mod tokens;

#[cfg(test)]
mod tests;

static PREFIX: &'static str = "🦪 ";

fn main() {
    let mut session = interpreter::Session::new();
    loop {
        // prompt
        print!("{}{}{}", PREFIX, session.get_var("PWD"), ">");

        io::stdout().flush().unwrap();

        // read
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // check if input is empty
        if input.trim().is_empty() {
            continue;
        }

        // eval
        let result = session.interpret(input);

        // printi 
        match result {
            Ok(vals) => print!("{}", vals),
            Err(err) => eprintln!("{}", err),
        }
    }
}

pub fn eval(result: interpreter::Value) -> Result<String, String> {
    match result {
        interpreter::
    }
}
