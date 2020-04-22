use regex::Regex;
use std::cmp::min;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    At,
    Amp,
    Ast,
    LParen,
    RParen,
    Plus,
    Minus,
    Mod,
    Eq,
    Pipe,
    Colon,
    LCurl,
    RCurl,
    LSq,
    RSq,
    Comma,
    Semi,
    Slash,
    NewLine,
    Var(String),
    Num(String),
    Str(String),
    // Path(String),
    // Param(String),
}

pub enum Case {
    Sym(&'static str, Token),
    Pat(&'static Regex, fn(String) -> Token),
}

lazy_static! {
    static ref SPACE_REGEX: Regex = Regex::new(r"^[ \t\r]+").unwrap();
    static ref VAR_REGEX: Regex = Regex::new(r"^\$[A-z0-9_]+").unwrap();
    static ref NUM_REGEX: Regex = Regex::new(r"^\d+(?:\.\d+)?").unwrap();
    static ref STR_REGEX: Regex = Regex::new("^\"[^\"]*\"").unwrap();
    // static ref PATH_REGEX: Regex =
    //     Regex::new(r"^/$|(^(?=/)|^.|^\.\.)(/(?=[^/\0])[^/\0]+)*/?").unwrap();
    static ref PARAM_REGEX: Regex = Regex::new("^--?[A-z]+(-[A-z]+)*").unwrap();
    static ref CASES: Vec<Case> = vec![
        Case::Sym("\n", Token::NewLine),
        Case::Sym("(", Token::LParen),
        Case::Sym(")", Token::RParen),
        Case::Sym("[", Token::LSq),
        Case::Sym("]", Token::RSq),
        Case::Sym("+", Token::Plus),
        Case::Sym("-", Token::Minus),
        Case::Sym("%", Token::Mod),
        Case::Sym("=", Token::Eq),
        Case::Sym("|", Token::Pipe),
        Case::Sym(":", Token::Colon),
        Case::Sym(";", Token::Semi),
        Case::Sym("{", Token::LCurl),
        Case::Sym("}", Token::RCurl),
        Case::Sym(",", Token::Comma),
        Case::Sym("*", Token::Ast),
        Case::Sym("/", Token::Slash),
        Case::Sym("@", Token::At),
        Case::Sym("&", Token::Amp),
        Case::Pat(&NUM_REGEX, Token::Num),
        Case::Pat(&VAR_REGEX, Token::Var),
        Case::Pat(&STR_REGEX, Token::Str),
        // Case::Pat(&PATH_REGEX, |s| Token::Path(s)),
    ];
}

pub struct Lexer<'a> {
    pub input: &'a str,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input }
    }

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        for case in CASES.iter() {
            match case {
                Case::Sym(s, token) => {
                    if let Some(some) = self.take_sym(s, token.clone()) {
                        return Some(some);
                    }
                }
                Case::Pat(pat, f) => {
                    if let Some(some) = self.take_regex(pat, *f) {
                        return Some(some);
                    }
                }
            }
        }

        None
    }

    fn take_sym(&mut self, s: &str, token: Token) -> Option<Token> {
        let input_len = self.input.len();
        let s_len = s.len();
        if input_len >= s_len && &self.input[..s_len] == s {
            self.skip_n(s_len);
            Some(token)
        } else {
            None
        }
    }

    fn take_regex(&mut self, pat: &Regex, f: fn(String) -> Token) -> Option<Token> {
        if let Some(mat) = pat.find(self.input) {
            assert_eq!(mat.start(), 0);
            let text = String::from(&self.input[..mat.end()]);
            self.skip_n(mat.end());
            Some(f(text))
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        if let Some(mat) = SPACE_REGEX.find(self.input) {
            assert_eq!(mat.start(), 0);
            self.skip_n(mat.end());
        }
    }

    fn skip_n(&mut self, n: usize) {
        self.input = &self.input[n..];
    }
}

#[derive(Debug)]
pub struct LexError<'a> {
    raw: &'a str,
}

impl std::fmt::Display for LexError<'_> {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        let n = min(10, self.raw.len());
        write!(f, "Unexpected token {}...", &self.raw[..n])
    }
}

pub fn tokenize<'a>(input: &'a str) -> Result<Vec<Token>, LexError<'a>> {
    let mut vec: Vec<Token> = vec![];
    let mut lexer = Lexer::new(input);
    while let Some(some) = lexer.next() {
        vec.push(some);
    }

    if lexer.input.is_empty() {
        Ok(vec)
    } else {
        Err(LexError { raw: input })
    }
}
