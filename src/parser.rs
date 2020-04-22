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
    Command(Box<Expr>,Vec<Expr>),
    Path(String),
    Param(String),
    Var(String),
    // UnaryOp(UnaryOp, Box<Expr>),
}

pub fn parse(ts: &[Token]) -> Result<Prog, String> {
    match parse_program(ts) {
        Some((prog, ts)) => match ts {
            [] => Ok(prog),
            _ => Err(format!("Unexpected tokens: {:?}.", ts)),
        },
        None => Err(String::from("Parse failed.")),
    }
}

fn parse_program(ts: &[Token]) -> Option<(Prog, &[Token])> {
    // If there aren't any tokens, end program.
    if let [] = ts {
        return Some((Prog::End, &[]));
    }

    if let Some((stmt, ts)) = parse_statement(ts) {
        if let [t, ..] = ts {
            if let Token::NewLine | Token::Semi = t {
                // Recurse to extract next line/command to return with the expression.
                if let Some((next, ts)) = parse_program(&ts[1..]) {
                    let prog = Prog::Stmt(Box::new(stmt), Box::new(next));
                    return Some((prog, ts));
                }
            }
        }
    }

    return None;
}

fn parse_statement(ts: &[Token]) -> Option<(Stmt, &[Token])> {
    if let [Token::Var(name), Token::Eq, ..] = ts {
        if let Some((expr, ts)) = parse_expression(&ts[2..]) {
            let stmt = Stmt::Assign(box name.clone(), box expr);
            return Some((stmt, ts));
        }
    }

    if let Some((expr, ts)) = parse_expression(ts) {
        return Some((Stmt::Expr(expr), ts));
    }

    return None;
}

fn parse_expression(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let [Token::Path(s), ..] = ts {
        let mut exprs = Vec::new();
        let mut ts = &ts[1..];
        loop {
            // Gather the following expressions as a vector for modifying the command.
            if let Some((expr,ts0)) = parse_factor(ts) {
                exprs.push(expr);
                ts = ts0;
                continue;
            }
            break;
        }
        return Some((Expr::Command(Box::new(Expr::Path(String::from(s))),exprs),ts));
    }
    
    if let Some((lfactor, ts)) = parse_term(ts) {
        let mut expr = lfactor;
        let mut ts = ts;
        loop {
            // Keep finding arithmetic stuff chained onto the right side and nest them in expressions.
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
            // Keep finding multiplicitive stuff chained onto the right side and nest them in expressions.
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

    if let Some(some) = parse_path(ts) {
        return Some(some);
    }

    if let Some(some) = parse_param(ts) {
        return Some(some);
    }
    
    if let [Token::Var(s), ..] = ts {
        return Some((Expr::Var(s.clone()), &ts[1..]));
    }

    if let [Token::LParen, ..] = ts {
        // If you find parentheses, recurse back to "Expression" level.
        if let Some((expr, ts)) = parse_expression(&ts[1..]) {
            if let [Token::RParen, ..] = ts {
                // Make sure you find a right parenthesis at the end of this nested expression.
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

fn parse_path(ts: &[Token]) -> Option<(Expr,&[Token])> {
    if let [Token::Path(s), ..] = ts {
        let expr = Expr::Path(String::from(s));
        return Some((expr, &ts[1..]));
    }

    return None;
}

fn parse_param(ts: &[Token]) -> Option<(Expr,&[Token])> {
    if let [Token::Param(s), ..] = ts {
        let expr = Expr::Param(String::from(s));
        return Some((expr, &ts[1..]));
    }

    return None;
}