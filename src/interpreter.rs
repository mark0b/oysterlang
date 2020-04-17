use crate::parser::{Expr, Program};
use std::fmt::Display;

pub enum Value {
    Str(String),
    Num(f64),
    Arr(),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Arr() => write!(f, "[]"),
        }
    }
}

pub fn interpret(prog: Program) -> Result<String, String> {
    eval_prog(prog)
}

fn eval_prog(prog: Program) -> Result<String, String> {
    match prog {
        Program::Statement(expr, next) => match eval_expr(*expr) {
            Ok(vcur) => match eval_prog(*next) {
                Ok(vnext) => Ok(format!("{}\n{}", vcur, vnext)),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Program::End => Ok(String::from("")),
    }
}

fn eval_expr(expr: Expr) -> Result<Value, String> {
    match expr {
        Expr::Add(box lexpr, box rexpr) => eval_expr_add(lexpr, rexpr),
        Expr::Sub(box lexpr, box rexpr) => eval_expr_sub(lexpr, rexpr),
        Expr::Mul(box lexpr, box rexpr) => eval_expr_mul(lexpr, rexpr),
        Expr::Div(box lexpr, box rexpr) => eval_expr_div(lexpr, rexpr),
        Expr::Mod(box lexpr, box rexpr) => eval_expr_mod(lexpr, rexpr),
        Expr::Num(n) => Ok(Value::Num(n)),
        Expr::Str(s) => Ok(Value::Str(s)),
        Expr::Arr() => Ok(Value::Arr()),
        // Expr::UnaryOp(_, expr) => match eval_expr(*expr) {
        //     Ok(_) => unimplemented!(),
        //     Err(err) => Err(err),
        // },
    }
}

fn eval_expr2(lexpr: Expr, rexpr: Expr) -> Result<(Value, Value), String> {
    match eval_expr(lexpr) {
        Ok(lval) => match eval_expr(rexpr) {
            Ok(rval) => Ok((lval, rval)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn eval_expr_div(lexpr: Expr, rexpr: Expr) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln / rn)),
        Ok(_) => Err(String::from("Can only divide numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mul(lexpr: Expr, rexpr: Expr) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln * rn)),
        Ok(_) => Err(String::from("Can only multiply numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_add(lexpr: Expr, rexpr: Expr) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln + rn)),
        Ok((Value::Str(ls), Value::Str(rs))) => Ok(Value::Str(format!("{}{}", ls, rs))),
        Ok(_) => Err(String::from("Can only add values of the same type.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_sub(lexpr: Expr, rexpr: Expr) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln - rn)),
        Ok(_) => Err(String::from("Can only subtract numbers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mod(lexpr: Expr, rexpr: Expr) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln % rn)),
        Ok(_) => Err(String::from("Can only mod numbers.")),
        Err(err) => Err(err),
    }
}
