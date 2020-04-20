use crate::tokens::Token;

pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Arr(),
    Num(f64),
    Str(String),
    // UnaryOp(UnaryOp, Box<Expr>),
}

pub enum Program {
    Statement(Box<Expr>, Box<Program>),
    End,
}

pub fn parse(ts: &[Token]) -> Result<Program, String> {
    match parse_program(ts) {
        Some((prog, ts)) => match ts {
            [] => Ok(prog),
            _ => Err(format!("Unexpected tokens: {:?}.", ts)),
        },
        None => Err(String::from("Parse failed.")),
    }
}

fn parse_program(ts: &[Token]) -> Option<(Program, &[Token])> {
    if let [] = ts {
        return Some((Program::End, &[]));
    }

    if let Some((expr, ts)) = parse_expression(ts) {
        if let [t, ..] = ts {
            if let Token::NewLine | Token::Semi = t {
                if let Some((next, ts)) = parse_program(&ts[1..]) {
                    let prog = Program::Statement(Box::new(expr), Box::new(next));
                    return Some((prog, ts));
                }
            }
        }
    }

    return None;
}

fn parse_expression(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some((lfactor, ts)) = parse_term(ts) {
        let mut expr = lfactor;
        let mut ts = ts;
        loop {
            if let [t, ..] = ts {
                if let Token::Plus | Token::Minus = t {
                    if let Some((rexpr, ts0)) = parse_term(&ts[1..]) {
                        expr = match t {
                            Token::Plus => Expr::Add(box expr, box rexpr),
                            Token::Minus => Expr::Sub(box expr, box rexpr),
                            _ => unreachable!(),
                        };
                        ts = ts0;
                        continue;
                    }
                }
            }
            break;
        }

        return Some((expr, ts));
    }

    return None;
}

fn parse_term(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some((lfactor, ts)) = parse_factor(ts) {
        let mut expr = lfactor;
        let mut ts = ts;
        loop {
            if let [t, ..] = ts {
                if let Token::Ast | Token::Slash | Token::Mod = t {
                    if let Some((rexpr, ts0)) = parse_factor(&ts[1..]) {
                        expr = match t {
                            Token::Ast => Expr::Mul(box expr, box rexpr),
                            Token::Slash => Expr::Div(box expr, box rexpr),
                            Token::Mod => Expr::Mod(box expr, box rexpr),
                            _ => unreachable!(),
                        };
                        ts = ts0;
                        continue;
                    }
                }
            }
            break;
        }

        return Some((expr, ts));
    }

    return None;
}

fn parse_factor(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some(some) = parse_num(ts) {
        return Some(some);
    }

    if let Some(some) = parse_str(ts) {
        return Some(some);
    }

    if let [Token::LParen, ..] = ts {
        if let Some((expr, ts)) = parse_expression(&ts[1..]) {
            if let [Token::RParen, ..] = ts {
                return Some((expr, &ts[1..]));
            }
        }
    }

    return None;
}

fn parse_num(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let [Token::Num(s), ..] = ts {
        if let Ok(n) = s.parse::<f64>() {
            return Some((Expr::Num(n), &ts[1..]));
        }
    }

    return None;
}

fn parse_str(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let [Token::Str(s), ..] = ts {
        let val = s.trim_matches('"');
        let expr = Expr::Str(String::from(val));
        return Some((expr, &ts[1..]));
    }

    return None;
}
