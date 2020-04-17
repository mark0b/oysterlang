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

    if let Some((expr, ts)) = parse_expr(ts) {
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

fn parse_expr(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some(some) = parse_additive_expr(ts) {
        return Some(some);
    }

    return None;
}

fn parse_term(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some(some) = parse_num(ts) {
        return Some(some);
    }

    if let Some(some) = parse_str(ts) {
        return Some(some);
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

fn shunting_yard(ts: &[Token]) -> Option<(Expr, &[Token])> {
    enum Item<'a> {
        Expr(Expr),
        Op(&'a Token),
    }
    let mut ts = ts;
    let mut out: Vec<Item> = vec![];
    let mut ops: Vec<&Token> = vec![];
    while let [t, ..] = ts {
        if let Some((expr, ts0)) = parse_term(ts) {
            out.push(Item::Expr(expr));
            ts = ts0;
            continue;
        }
        if is_op(t) {
            loop {
                if let Some(head) = ops.last() {
                    let p = precedence;
                    if p(head) > p(t) || (p(head) == p(t) && is_left_assoc(t)) {
                        let head = ops.pop().unwrap();
                        out.push(Item::Op(head));
                        continue;
                    }
                }
                break;
            }
            out.push(Item::Op(t));
            ts = &ts[1..];
            continue;
        }
        if let Token::NewLine | Token::Semi = t {
            break;
        }
        unreachable!();
    }
    while let Some(t) = ops.pop() {
        out.push(Item::from(t));
    }

    fn emit(lexpr: Expr, items: &[Item]) -> Expr {
        match items {
            [Item::Expr(rexpr), Item::Op(t), ..] => 
        }
    }
}

fn parse_additive_expr(ts: &[Token]) -> Option<(Expr, &[Token])> {
    fn emit(ops: &[Token], exprs: &[Expr]) -> Option<Expr> {}

    if let Some((lexpr, mut ts)) = parse_multiplicative_expr(ts) {
        let mut ops: Vec<&Token> = vec![];
        let mut exprs: Vec<Expr> = vec![];
        while let Some((op, rexpr, ts0)) = parse_additive_rexpr(ts) {
            ops.push(op);
            exprs.push(rexpr);
            ts = ts0;
        }

        while !exprs.is_empty() {}

        if exprs.len() != 1 {
            unreachable!();
        }

        if let Some((rexpr, ts)) = parse_additive_expr(&ts[1..]) {
            let expr = match t {
                Token::Plus => Expr::Add(box lexpr, box rexpr),
                Token::Minus => Expr::Sub(box lexpr, box rexpr),
                _ => unreachable!(),
            };
            return Some((expr, ts));
        }

        if let [t, ..] = ts {
            if let Token::Plus | Token::Minus = t {
                if let Some((rexpr, ts)) = parse_additive_expr(&ts[1..]) {
                    let expr = match t {
                        Token::Plus => Expr::Add(box lexpr, box rexpr),
                        Token::Minus => Expr::Sub(box lexpr, box rexpr),
                        _ => unreachable!(),
                    };
                    return Some((expr, ts));
                }
            }
        }
        return Some((lexpr, ts));
    }

    return None;
}

fn parse_additive_rexpr(ts: &[Token]) -> Option<(&Token, Expr, &[Token])> {
    if let [t, ..] = ts {
        if let Token::Plus | Token::Minus = t {
            if let Some((expr, ts)) = parse_multiplicative_expr(&ts[1..]) {
                return Some((t, expr, ts));
            }
        }
    }

    return None;
}

fn parse_multiplicative_expr(ts: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some((lexpr, ts)) = parse_term(ts) {
        if let [t, ..] = ts {
            if let Token::Ast | Token::Slash | Token::Mod = t {
                if let Some((rexpr, ts)) = parse_multiplicative_expr(&ts[1..]) {
                    let expr = match t {
                        Token::Ast => Expr::Mul(box lexpr, box rexpr),
                        Token::Slash => Expr::Div(box lexpr, box rexpr),
                        Token::Mod => Expr::Mod(box lexpr, box rexpr),
                        _ => unreachable!(),
                    };
                    return Some((expr, ts));
                }
            }
        }
        return Some((lexpr, ts));
    }

    return None;
}

fn is_op(t: &Token) -> bool {
    match t {
        Token::Plus | Token::Minus | Token::Ast | Token::Slash | Token::Mod => true,
        _ => false,
    }
}

fn is_left_assoc(t: &Token) -> bool {
    match t {
        Token::Plus | Token::Minus | Token::Ast | Token::Slash | Token::Mod => true,
        _ => false,
    }
}

fn is_right_assoc(t: &Token) -> bool {
    match t {
        _ => false,
    }
}

fn precedence(t: &Token) -> usize {
    match t {
        Token::Ast | Token::Slash | Token::Mod => 2,
        Token::Plus | Token::Minus => 1,
        _ => 0,
    }
}
