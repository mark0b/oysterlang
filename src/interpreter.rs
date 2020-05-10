use crate::parser::*;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::process::{self, Command, Stdio};

type Env = HashMap<String, Value>;

#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Arr(),
    Pipeline(process::Output),
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Arr() => write!(f, "[]"),
            Value::Pipeline(out) => write!(
                f,
                "{}",
                match String::from_utf8(out.clone().stdout) {
                    Ok(out) => String::from(out.trim_end()),
                    Err(err) => format!("{}", err),
                }
            ),
            Value::Void => write!(f, ""),
        }
    }
}

pub fn interpret(prog: &Prog) -> Result<String, String> {
    let mut env = Env::new();

    for (k, v) in std::env::vars() {
        env.insert(k, Value::Str(v));
    }

    exec_prog(&prog, &env)
}

fn exec_prog(prog: &Prog, env: &Env) -> Result<String, String> {
    match prog {
        Prog::Stmt(box stmt, box next) => match exec_stmt(stmt, env) {
            Ok((vcur, env)) => match exec_prog(next, &env) {
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

fn exec_stmt(stmt: &Stmt, env: &Env) -> Result<(String, Env), String> {
    match stmt {
        Stmt::Assign(box name, box expr) => match eval_expr(expr, env) {
            Ok((val, env)) => {
                let mut env = env.clone();
                env.insert(String::from(name), val);
                Ok((String::from(""), env))
            }
            Err(err) => Err(err),
        },
        Stmt::Expr(expr) => match eval_expr(expr, &env) {
            Ok((val, env)) => Ok((format!("{}", val), env.clone())),
            Err(err) => Err(err),
        },
    }
}

fn eval_expr(expr: &Expr, env: &Env) -> Result<(Value, Env), String> {
    match expr {
        Expr::Add(box lexpr, box rexpr) => eval_expr_add(lexpr, rexpr, env),
        Expr::Sub(box lexpr, box rexpr) => eval_expr_sub(lexpr, rexpr, env),
        Expr::Mul(box lexpr, box rexpr) => eval_expr_mul(lexpr, rexpr, env),
        Expr::Div(box lexpr, box rexpr) => eval_expr_div(lexpr, rexpr, env),
        Expr::Mod(box lexpr, box rexpr) => eval_expr_mod(lexpr, rexpr, env),
        Expr::Num(n) => Ok((Value::Num(*n),env.clone())),
        Expr::Str(s) => Ok((Value::Str(String::from(s)), env.clone())),
        Expr::Arr() => Ok((Value::Arr(), env.clone())),
        Expr::Param(s) => Ok((Value::Str(String::from(s)), env.clone())),
        Expr::Path(s) => Ok((Value::Str(String::from(s)), env.clone())),
        Expr::Var(s) =>
            match env.get(s) {
                Some(val) => Ok((val.clone(), env.clone())),
                None => Ok((Value::Void, env.clone())),
            }
        Expr::Cmd(_, _) => match eval_command(expr, env) {
            Ok((output, env)) => return Ok((output, env)),
            Err(err) => Err(err),
        }

        // Expr::UnaryOp(_, expr) => match eval_expr(*expr) {
        //     Ok(_) => unimplemented!(),
        //     Err(err) => Err(err),
        // },
    }
}

fn eval_expr2(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<(Value, Value), String> {
    match eval_expr(lexpr, env) {
        Ok((lval, env)) => match eval_expr(rexpr, &env) {
            Ok((rval, _env)) => Ok((lval, rval)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn eval_expr_div(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<(Value, Env), String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok((Value::Num(ln / rn), env.clone())),
        Ok(_) => Err(String::from("Can only divide numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mul(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<(Value, Env), String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok((Value::Num(ln * rn), env.clone())),
        Ok(_) => Err(String::from("Can only multiply numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_add(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<(Value, Env), String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok((Value::Num(ln + rn), env.clone())),
        Ok((Value::Str(ls), Value::Str(rs))) => {
            Ok((Value::Str(format!("{}{}", ls, rs)), env.clone()))
        }
        Ok(_) => Err(String::from("Can only add values of the same type.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_sub(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<(Value, Env), String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok((Value::Num(ln - rn), env.clone())),
        Ok(_) => Err(String::from("Can only subtract numbers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mod(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<(Value, Env), String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok((Value::Num(ln % rn), env.clone())),
        Ok(_) => Err(String::from("Can only mod numbers.")),
        Err(err) => Err(err),
    }
}

fn eval_command(expr: &Expr, env: &Env) -> Result<(Value, Env), String> {
    if let Expr::Cmd(box Expr::Path(s), args) = expr {
        let mut command = Command::new(s);
        match args
            .iter()
            .map(|a| {
                match eval_expr(a, env) {
                    Ok((val, _env)) => Ok(val), //throwing away any env changes that occur here.
                    Err(err) => Err(err),
                }
            })
            .collect::<Result<Vec<Value>, String>>()
        {
            Ok(vals) => {
                match command
                    .args(vals.iter().map(|a| format!("{}", a)))
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::piped()) //if stdout is inherited output stream will be empty, if it is piped, we don't have a prompt for input.
                    .output()
                {
                    Ok(out) => {
                        let mut env = env.clone();
                        if let Some(code) = out.status.code() {
                            env.insert(String::from("$?"), Value::Num(code as f64));
                        }
                        return Ok((Value::Pipeline(out), env));
                        // match String::from_utf8(out.stdout) {
                        //     Ok(out) => return Ok((Value::Str(String::from(out.trim_end())), env)),
                        //     Err(err) => return Err(format!("{}", err)),
                        // }
                    }
                    Err(err) => {
                        return Err(format!("{}", err));
                    }
                }
            }
            Err(err) => return Err(err),
        }
    }
    return Err(String::from("Failed to evaluate command."));
}
