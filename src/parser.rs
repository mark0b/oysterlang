use crate::tokens::Token;

pub enum Prog {
    Stmt(Box<Stmt>, Box<Prog>),
    End,
}

pub enum Stmt {
    Assign(Box<String>, Box<Expr>),
    Expr(Expr),
}

pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Arr(),
    Num(f64),
    Str(String),
    Var(String),
    // UnaryOp(UnaryOp, Box<Expr>),
}

pub fn parse(ts: &[Token]) -> Result<Prog, String> {
    match parse_prog(ts) {
        Some((prog, ts)) => match ts {
            [] => Ok(prog),
            _ => Err(format!("Unexpected tokens: {:?}.", ts)),
        },
        None => Err(String::from("Parse failed.")),
    }
}

fn parse_prog(ts: &[Token]) -> Option<(Prog, &[Token])> {
    if let [] = ts {
        return Some((Prog::End, &[]));
    }

    if let Some((stmt, ts)) = parse_stmt(ts) {
        if let [t, ..] = ts {
            if let Token::NewLine | Token::Semi = t {
                if let Some((next, ts)) = parse_prog(&ts[1..]) {
                    let prog = Prog::Stmt(Box::new(stmt), Box::new(next));
                    return Some((prog, ts));
                }
            }
        }
    }

    return None;
}

fn parse_stmt(ts: &[Token]) -> Option<(Stmt, &[Token])> {
    if let [Token::Var(name), Token::Eq, ..] = ts {
        if let Some((expr, ts)) = parse_expr(&ts[2..]) {
            let stmt = Stmt::Assign(box name.clone(), box expr);
            return Some((stmt, ts));
        }
    }

    if let Some((expr, ts)) = parse_expr(ts) {
        return Some((Stmt::Expr(expr), ts));
    }

    return None;
}

fn parse_expr(ts: &[Token]) -> Option<(Expr, &[Token])> {
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

    if let [Token::Var(s), ..] = ts {
        return Some((Expr::Var(s.clone()), &ts[1..]));
    }

    if let [Token::LParen, ..] = ts {
        if let Some((expr, ts)) = parse_expr(&ts[1..]) {
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
