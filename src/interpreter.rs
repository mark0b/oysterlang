use crate::parser::*;
use std::{collections::HashMap, fmt::Display};

type Env = HashMap<String, Value>;

fn get_env() -> Env {
    let mut env = Env::new();
    for (k, v) in std::env::vars() {
        env.insert(k, Value::Str(v));
    }
    env
}

#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Arr(),
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Arr() => write!(f, "[]"),
            Value::Void => write!(f, ""),
        }
    }
}

pub fn interpret(prog: Prog) -> Result<String, String> {
    exec_prog(prog, get_env())
}

fn exec_prog(prog: Prog, env: Env) -> Result<String, String> {
    match prog {
        Prog::Stmt(box stmt, box next) => match exec_stmt(stmt, env) {
            Ok((vcur, env)) => match exec_prog(next, env) {
                Ok(vnext) => match vcur.is_empty() {
                    true => Ok(format!("{}", vnext)),
                    false => Ok(format!("{}\n{}", vcur, vnext)),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Prog::End => Ok(String::from("")),
    }
}

fn exec_stmt(stmt: Stmt, env: Env) -> Result<(String, Env), String> {
    match stmt {
        Stmt::Assign(box name, box expr) => match eval_expr(expr, env.clone()) {
            Ok(val) => {
                let mut env = env.clone();
                env.insert(name, val);
                Ok((String::from(""), env))
            }
            Err(err) => Err(err),
        },
        Stmt::Expr(expr) => match eval_expr(expr, env.clone()) {
            Ok(val) => Ok((format!("{}", val), env.clone())),
            Err(err) => Err(err),
        },
    }
}

fn eval_expr(expr: Expr, env: Env) -> Result<Value, String> {
    match expr {
        Expr::Add(box lexpr, box rexpr) => eval_expr_add(lexpr, rexpr, env),
        Expr::Sub(box lexpr, box rexpr) => eval_expr_sub(lexpr, rexpr, env),
        Expr::Mul(box lexpr, box rexpr) => eval_expr_mul(lexpr, rexpr, env),
        Expr::Div(box lexpr, box rexpr) => eval_expr_div(lexpr, rexpr, env),
        Expr::Mod(box lexpr, box rexpr) => eval_expr_mod(lexpr, rexpr, env),
        Expr::Num(n) => Ok(Value::Num(n)),
        Expr::Str(s) => Ok(Value::Str(s)),
        Expr::Arr() => Ok(Value::Arr()),
        Expr::Param(s) => Ok(Value::Str(s)),
        Expr::Path(s) => Ok(Value::Str(s)),
        Expr::Command(_,_) => Ok(Value::Str(String::new())),//Temporarily putting this hear as a placeholder.
        Expr::Var(s) => 
            match env.get(&s) {
                Some(val) => Ok(val.clone()),
                None => Ok(Value::Void)
            }
        // Expr::UnaryOp(_, expr) => match eval_expr(*expr) {
        //     Ok(_) => unimplemented!(),
        //     Err(err) => Err(err),
        // },
    }
}

fn eval_expr2(lexpr: Expr, rexpr: Expr, env: Env) -> Result<(Value, Value), String> {
    match eval_expr(lexpr, env.clone()) {
        Ok(lval) => match eval_expr(rexpr, env.clone()) {
            Ok(rval) => Ok((lval, rval)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn eval_expr_div(lexpr: Expr, rexpr: Expr, env: Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln / rn)),
        Ok(_) => Err(String::from("Can only divide numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mul(lexpr: Expr, rexpr: Expr, env: Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln * rn)),
        Ok(_) => Err(String::from("Can only multiply numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_add(lexpr: Expr, rexpr: Expr, env: Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln + rn)),
        Ok((Value::Str(ls), Value::Str(rs))) => Ok(Value::Str(format!("{}{}", ls, rs))),
        Ok(_) => Err(String::from("Can only add values of the same type.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_sub(lexpr: Expr, rexpr: Expr, env: Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln - rn)),
        Ok(_) => Err(String::from("Can only subtract numbers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mod(lexpr: Expr, rexpr: Expr, env: Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln % rn)),
        Ok(_) => Err(String::from("Can only mod numbers.")),
        Err(err) => Err(err),
    }
}
