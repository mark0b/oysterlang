use crate::ast;
use crate::parser;
use crate::parser::num;

pub fn parse(cursor: &parser::Cursor) -> parser::Result<ast::LNum> {
    let root = cursor;
    if let Ok((num, cursor)) = parse_expr(&cursor) {
        let expr = ast::LNum::Expr(root.clone(), Box::new(num));
        return Ok((expr, cursor));
    }
    if let Ok(ok) = parse_float(&cursor) {
        return Ok(ok);
    }
    if let Ok(ok) = parse_int(&cursor) {
        return Ok(ok);
    }
    let err = parser::Error::new(cursor, "num");
    Err(err)
}

fn parse_float(cursor: &parser::Cursor) -> parser::Result<ast::LNum> {
    lazy_static! {
        static ref FLOAT: regex::Regex = regex::Regex::new(r"^\d+\.\d+").unwrap();
    }
    match cursor.clone().skip_whitespace().take(&FLOAT, "float") {
        Ok((s, context)) => {
            let f = s.parse::<f64>().unwrap();
            let num = ast::LNum::Float(cursor.clone(), f);
            Ok((num, context))
        }
        Err(err) => Err(err),
    }
}

fn parse_int(cursor: &parser::Cursor) -> parser::Result<ast::LNum> {
    lazy_static! {
        static ref INT: regex::Regex = regex::Regex::new(r"^\d+").unwrap();
    }
    match cursor.clone().skip_whitespace().take(&INT, "int") {
        Ok((s, context)) => {
            let i = s.parse::<isize>().unwrap();
            let num = ast::LNum::Int(cursor.clone(), i);
            Ok((num, context))
        }
        Err(err) => Err(err),
    }
}

pub fn parse_expr(cursor: &parser::Cursor) -> parser::Result<ast::Num> {
    lazy_static! {
        static ref LEFT_PAREN: regex::Regex = regex::Regex::new(r"\(").unwrap();
        static ref RIGHT_PAREN: regex::Regex = regex::Regex::new(r"\)").unwrap();
    }
    if let Ok((_, cursor)) = cursor.clone().skip_whitespace().take(&LEFT_PAREN, "expr") {
        if let Ok((n, cursor)) = num::parse(&cursor) {
            if let Ok((_, cursor)) = cursor.skip_whitespace().take(&RIGHT_PAREN, ")") {
                return Ok((n, cursor));
            }
        }
    }
    Err(parser::Error::new(cursor, "number"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_float_without_preceding_digit() {
        let cursor = parser::Cursor::new("");
        parse_num(cursor: parser::Cursor)
    }

    #[test]
    fn test_parse_int() {}

    #[test]
    fn test_parse_expr() {}

    #[test]
    fn test_parse_add() {}

    #[test]
    fn test_parse_sub() {}
}
