use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            line: 0,
            character: 0,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} char {}", self.line, self.character)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub posn: Position,
}

impl Context {
    pub fn add_chars(self, n: usize) -> Context {
        let posn = Position {
            character: self.posn.character + n,
            ..self.posn
        };
        Context { posn, ..self }
    }
    pub fn add_lines(self, n: usize) -> Context {
        let posn = Position {
            character: 0,
            line: self.posn.line + n,
        };
        Context { posn, ..self }
    }
    pub fn new() -> Context {
        Context {
            posn: Position::new(),
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.posn)
    }
}
