use crate::ast;
use crate::parser;
use crate::parser::lnum;
use regex;

pub fn parse_add(cursor: &parser::Cursor) -> parser::Result<ast::Num> {
    lazy_static! {
        static ref ADD: regex::Regex = regex::Regex::new(r"^\+").unwrap();
    }
    let root = cursor;
    if let Ok((lnum, cursor)) = lnum::parse(cursor) {
        if let Ok((_, cursor)) = cursor.take(&ADD, "+") {
            if let Ok((rnum, cursor)) = parse(&cursor) {
                let num = ast::Num::Add(root.clone(), lnum, Box::new(rnum));
                return Ok((num, cursor));
            }
        }
    }
    Err(parser::Error::new(cursor, "add"))
}

pub fn parse_sub(cursor: &parser::Cursor) -> parser::Result<ast::Num> {
    lazy_static! {
        static ref SUB: regex::Regex = regex::Regex::new(r"^\-").unwrap();
    }
    let root = cursor;
    if let Ok((lnum, cursor)) = lnum::parse(cursor) {
        if let Ok((_, cursor)) = cursor.take(&SUB, "-") {
            if let Ok((rnum, cursor)) = parse(&cursor) {
                let num = ast::Num::Sub(root.clone(), lnum, Box::new(rnum));
                return Ok((num, cursor));
            }
        }
    }
    Err(parser::Error::new(cursor, "sub"))
}

pub fn parse(cursor: &parser::Cursor) -> parser::Result<ast::Num> {
    if let Ok((atom, cursor)) = lnum::parse(cursor) {
        let num = ast::Num::Atom(atom);
        return Ok((num, cursor));
    }
    if let Ok(ok) = parse_add(cursor) {
        return Ok(ok);
    }
    if let Ok(ok) = parse_sub(cursor) {
        return Ok(ok);
    }
    let err = parser::Error::new(cursor, "num");
    Err(err)
}
