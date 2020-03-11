use crate::context;
use crate::parser;

#[derive(Debug)]
pub enum LNum {
    Float(parser::Cursor, f64),
    Int(parser::Cursor, isize),
    Expr(parser::Cursor, Box<Num>),
}

#[derive(Debug)]
pub enum Num {
    Atom(LNum),
    Add(parser::Cursor, LNum, Box<Num>),
    Sub(parser::Cursor, LNum, Box<Num>),
}

#[derive(Debug)]
pub enum Str {
    Literal(parser::Cursor, String),
    Concat(parser::Cursor, Box<Str>, Box<Str>),
}

#[derive(Debug)]
pub enum Expr {
    Num(Num),
    Str(String),
}
