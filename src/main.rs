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
    loop {
        // prompt
        print!("{}", PREFIX);
        if let Ok(dir) = env::current_dir() {
            print!("{}", dir.to_str().unwrap());
        }
        print!("{}", ">");

        io::stdout().flush().unwrap();

        // read
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // check if input is empty
        if input.trim_end().len() == 0 {
            continue;
        }

        // eval
        let result = eval(&input);

        // print
        match result {
            Ok(ok) => print!("{}", ok),
            Err(err) => eprintln!("{}", err),
        }
    }
}

pub fn eval(input: &str) -> Result<String, String> {
    match tokens::tokenize(input) {
        Ok(ts) => match parser::parse(&ts) {
            Ok(prog) => interpreter::interpret(&prog),
            Err(err) => Err(err),
        },
        Err(err) => Err(format!("{}", err)),
    }
}
