use crate::ast;
use regex;
use std::cmp::max;
use std::fmt;
use std::rc::Rc;
use std::result;

mod lnum;
mod num;

#[derive(Clone)]
pub struct Cursor {
    input: Rc<String>,
    index: usize,
    line: usize,
    character: usize,
}

impl fmt::Debug for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.line, self.character)
    }
}

impl Cursor {
    fn skip_whitespace(self) -> Cursor {
        let s = self.head();
        if s.is_empty() {
            self
        } else {
            match &s[..1] {
                " " | "\t" => Cursor {
                    index: self.index + 1,
                    character: self.character + 1,
                    ..self.clone()
                },
                "\r" => Cursor {
                    index: self.index + 1,
                    ..self.clone()
                },
                "\n" => Cursor {
                    index: self.index + 1,
                    character: 0,
                    line: self.line + 1,
                    ..self.clone()
                },
                _ => self.clone(),
            }
        }
    }

    pub fn take(&self, pattern: &'static regex::Regex, expected: &'static str) -> Result<&str> {
        let s = self.head();
        match pattern.find(self.head()) {
            Some(mat) => {
                let (start, stop) = (mat.start(), mat.end());
                let s = &s[start..stop];
                let n = stop - start;
                let cursor = Cursor {
                    character: self.character + n,
                    index: self.index + n,
                    ..self.clone()
                };
                Ok((s, cursor.skip_whitespace()))
            }
            None => {
                let err = Error {
                    cursor: self.clone(),
                    expected,
                };
                Err(err)
            }
        }
    }

    pub fn head(&self) -> &str {
        let s = &*self.input;
        &s[self.index..]
    }

    pub fn new(input: &str) -> Cursor {
        Cursor {
            input: Rc::new(String::from(input)),
            line: 0,
            character: 0,
            index: 0,
        }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self.head();
        let len = max(s.len(), 5);
        write!(
            f,
            "{}..., line {} char {}.",
            &s[..len],
            self.line,
            self.character
        )
    }
}

#[derive(Clone)]
pub struct Error {
    expected: &'static str,
    cursor: Cursor,
}

impl Error {
    fn new(cursor: &Cursor, expected: &'static str) -> Error {
        Error {
            cursor: cursor.clone(),
            expected,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let found = &self.cursor.head();
        let len = max(found.len(), 5);
        write!(f, "Expected {} at {}.", &self.expected[..len], &self.cursor)
    }
}

pub type Result<T> = result::Result<(T, Cursor), Error>;

pub fn parse(input: &str) -> ast::Expr {
    let cursor = Cursor::new(input);
    let (num, _) = num::parse(&cursor).unwrap();
    ast::Expr::Num(num)
}
