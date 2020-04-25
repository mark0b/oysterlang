use crate::parser::*;
use std::{collections::HashMap};
use std::fmt::{Display, Formatter, self};
use std::process::{self,Command,Stdio};

type Env = HashMap<String, Value>;

#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Arr(),
    Proc(process::ExitStatus),
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Proc(out) => match out.code() {
                Some(code) => write!(f, "{}", code),
                None => write!(f, "No exit code."),
            }, //TODO
            Value::Str(s) => write!(f, "{}", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Arr() => write!(f, "[]"),
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
            Ok(val) => {
                let mut env = env.clone();
                env.insert(String::from(name), val);
                Ok((String::from(""), env))
            }
            Err(err) => Err(err),
        },
        Stmt::Expr(expr) => match eval_expr(expr, &env) {
            Ok(val) => Ok((format!("{}", val), env.clone())),
            Err(err) => Err(err),
        },
    }
}

fn eval_expr(expr: &Expr, env: &Env) -> Result<Value, String> {
    match expr {
        Expr::Add(box lexpr, box rexpr) => eval_expr_add(lexpr, rexpr, env),
        Expr::Sub(box lexpr, box rexpr) => eval_expr_sub(lexpr, rexpr, env),
        Expr::Mul(box lexpr, box rexpr) => eval_expr_mul(lexpr, rexpr, env),
        Expr::Div(box lexpr, box rexpr) => eval_expr_div(lexpr, rexpr, env),
        Expr::Mod(box lexpr, box rexpr) => eval_expr_mod(lexpr, rexpr, env),
        Expr::Num(n) => Ok(Value::Num(*n)),
        Expr::Str(s) => Ok(Value::Str(String::from(s))),
        Expr::Arr() => Ok(Value::Arr()),
        Expr::Param(s) => Ok(Value::Str(String::from(s))),
        Expr::Path(s) => Ok(Value::Str(String::from(s))),
        Expr::Command(box expr, args) => eval_command(expr, args, env),
        Expr::Var(s) => 
            match env.get(s) {
                Some(val) => Ok(val.clone()),
                None => Ok(Value::Void)
            }

        // Expr::UnaryOp(_, expr) => match eval_expr(*expr) {
        //     Ok(_) => unimplemented!(),
        //     Err(err) => Err(err),
        // },
    }
}

fn eval_expr2(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<(Value, Value), String> {
    match eval_expr(lexpr, env) {
        Ok(lval) => match eval_expr(rexpr, env) {
            Ok(rval) => Ok((lval, rval)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn eval_expr_div(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln / rn)),
        Ok(_) => Err(String::from("Can only divide numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mul(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln * rn)),
        Ok(_) => Err(String::from("Can only multiply numers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_add(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln + rn)),
        Ok((Value::Str(ls), Value::Str(rs))) => Ok(Value::Str(format!("{}{}", ls, rs))),
        Ok(_) => Err(String::from("Can only add values of the same type.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_sub(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln - rn)),
        Ok(_) => Err(String::from("Can only subtract numbers.")),
        Err(err) => Err(err),
    }
}

fn eval_expr_mod(lexpr: &Expr, rexpr: &Expr, env: &Env) -> Result<Value, String> {
    match eval_expr2(lexpr, rexpr, env) {
        Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln % rn)),
        Ok(_) => Err(String::from("Can only mod numbers.")),
        Err(err) => Err(err),
    }
}

fn eval_command(expr: &Expr,args: &Vec<Expr>, env: &Env) -> Result<Value, String> {
    if let Expr::Path(s) = expr {
        let mut command = Command::new(s);
        match args.iter()
            .map(|a| eval_expr(a,env))
            .collect::<Result<Vec<Value>,String>>()
        {
            Ok(vals) => {command.args(vals.iter().map(|a| format!("{}",a)));},
            Err(err) => return Err(err),
        }
        let proc = match command.status() {
            Ok(proc) => proc,
            Err(err) => return Err(format!("{}",err)),
        };
        if proc.success() {
            return Ok(Value::Proc(proc));
        } else {
            return Err(String::from("Command failed to execute."));
        }
    }
    return Err(String::from("Failed to evaluate command."));
}
