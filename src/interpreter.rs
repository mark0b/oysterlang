use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::process::{self, Command, Stdio};

use crate::parser::{self,Stmt,Expr,Prog};
use crate::tokens;

type Env = HashMap<String, Value>;

#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Arr(Vec<Value>),
    Pipeline(process::Output),
    Void,
}

// TODO: try and figure out the big-o complexity of this method.
//       you should be able to make a O(n) implementation keeping your
//       recursive pattern if we change Arr to a linked list
//       https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
impl Value {
    fn push(&self, next: Value) -> Value {
        match self {
            Value::Void => next,
            Value::Arr(vals) => {
                let mut vals = vals.clone();
                match next {
                    Value::Void => (),
                    Value::Arr(nexts) => vals.extend(nexts),
                    _ => vals.push(next),
                }
                Value::Arr(vals)
            }
            _ => match next {
                    Value::Void => self.clone(),
                    Value::Arr(nexts) => {
                        let vals = vec!(self.clone());
                        vals.extend(nexts);
                        Value::Arr(vals)
                    }
                    _ => Value::Arr(vec!(self.clone(),next)),
                }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Arr(_vals) => write!(f, "[]"),
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

pub struct Session {
    env: Env,
}

impl Session {
    pub fn new() -> Session {
        let mut env = Env::new();
        for (k, v) in std::env::vars() {
            env.insert(k, Value::Str(v));
        }
        if let Ok(pwd) = std::env::current_dir() {
            if let Some(pwd) = pwd.to_str() {
                env.insert(
                    String::from("PWD"), // TODO: we shouldn't have `$` in the env var names.  the ones we inherit wont.
                    Value::Str(String::from(pwd)),
                );
            }
        }
        Session {env}
    }

    pub fn get_var(&self, s: &str) -> Value {
        match self.env.get(s) {
            Some(val) => val.clone(),
            _ => Value::Void,
        }
    }

    pub fn interpret(&mut self, input: String) {
        match tokens::tokenize(&input) {
            Ok(ts) => match parser::parse(&ts) {
                Ok(prog) => match self.exec_prog(&prog) {
                    Ok((value, env)) => {
                        self.env = env;
                        Ok(value)
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn exec_prog(&self, prog: &Prog) -> Result<(Value, Env), String> {
        match prog {
            Prog::Stmt(box stmt, box next) => match self.exec_stmt(stmt) {
                Ok(vcur) => match self.exec_prog(next) {
                    Ok(vnext) => Ok(vcur.push(vnext)),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Prog::End => Ok(Value::Void),
        }
    }
    
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Assign(box name, box expr) => match self.eval_expr(expr) {
                Ok(val) => {
                    self.env.insert(String::from(name), val);
                    Ok(Value::Void)
                }
                Err(err) => Err(err),
            },
            Stmt::Expr(expr) => self.eval_expr(expr),
        }
    }
    
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Add(box lexpr, box rexpr) => self.eval_expr_add(lexpr, rexpr),
            Expr::Sub(box lexpr, box rexpr) => self.eval_expr_sub(lexpr, rexpr),
            Expr::Mul(box lexpr, box rexpr) => self.eval_expr_mul(lexpr, rexpr),
            Expr::Div(box lexpr, box rexpr) => self.eval_expr_div(lexpr, rexpr),
            Expr::Mod(box lexpr, box rexpr) => self.eval_expr_mod(lexpr, rexpr),
            Expr::Num(n) => Ok(Value::Num(*n)),
            Expr::Str(s) => Ok(Value::Str(String::from(s))),
            Expr::Arr() => Ok(Value::Arr(vec!())),
            Expr::Param(s) => Ok(Value::Str(String::from(s))),
            Expr::Path(s) => Ok(Value::Str(String::from(s))),
            Expr::Var(s) =>
                match self.env.get(s) {
                    Some(val) => Ok(val.clone()),
                    None => Ok(Value::Void),
                }
            Expr::Cmd(_, _) => match self.eval_cmd(expr) {
                Ok(output) => return Ok(output),
                Err(err) => Err(err),
            }
    
            // Expr::UnaryOp(_, expr) => match eval_expr(*expr) {
            //     Ok(_) => unimplemented!(),
            //     Err(err) => Err(err),
            // },
        }
    }
    
    fn eval_expr2(&mut self, lexpr: &Expr, rexpr: &Expr) -> Result<(Value, Value), String> {
        match self.eval_expr(lexpr) {
            Ok(lval) => match self.eval_expr(rexpr) {
                Ok(rval) => Ok((lval, rval)),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    fn eval_expr_div(&mut self, lexpr: &Expr, rexpr: &Expr) -> Result<Value, String> {
        match self.eval_expr2(lexpr, rexpr) {
            Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln / rn)),
            Ok(_) => Err(String::from("Can only divide numers.")),
            Err(err) => Err(err),
        }
    }
    
    fn eval_expr_mul(&mut self, lexpr: &Expr, rexpr: &Expr) -> Result<Value, String> {
        match self.eval_expr2(lexpr, rexpr) {
            Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln * rn)),
            Ok(_) => Err(String::from("Can only multiply numers.")),
            Err(err) => Err(err),
        }
    }
    
    fn eval_expr_add(&mut self, lexpr: &Expr, rexpr: &Expr) -> Result<Value, String> {
        match self.eval_expr2(lexpr, rexpr) {
            Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln + rn)),
            Ok((Value::Str(ls), Value::Str(rs))) => {
                Ok(Value::Str(format!("{}{}", ls, rs)))
            }
            Ok(_) => Err(String::from("Can only add values of the same type.")),
            Err(err) => Err(err),
        }
    }
    
    fn eval_expr_sub(&mut self, lexpr: &Expr, rexpr: &Expr) -> Result<Value, String> {
        match self.eval_expr2(lexpr, rexpr) {
            Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln - rn)),
            Ok(_) => Err(String::from("Can only subtract numbers.")),
            Err(err) => Err(err),
        }
    }
    
    fn eval_expr_mod(&mut self, lexpr: &Expr, rexpr: &Expr) -> Result<Value, String> {
        match self.eval_expr2(lexpr, rexpr) {
            Ok((Value::Num(ln), Value::Num(rn))) => Ok(Value::Num(ln % rn)),
            Ok(_) => Err(String::from("Can only mod numbers.")),
            Err(err) => Err(err),
        }
    }
    
    fn eval_cmd(&mut self, expr: &Expr) -> Result<Value, String> {
        if let Expr::Cmd(box Expr::Path(s), args) = expr {
            match args
                .iter()
                .map(|a| {
                    match self.eval_expr(a) {
                        Ok(val) => Ok(val),
                        Err(err) => Err(err),
                    }
                })
                .collect::<Result<Vec<Value>, String>>()
            {
                Ok(vals) => {
                    match Command::new(s)
                        .args(vals.iter().map(|a| format!("{}", a)))
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit()) //if stdout is inherited output stream will be empty, if it is piped, we don't have a prompt for input.
                        .stderr(Stdio::inherit())
                        .output()
                    {
                        Ok(out) => {
                            if let Some(code) = out.status.code() {
                                self.env.insert(String::from("$?"), Value::Num(code as f64));
                            }
                            return Ok(Value::Pipeline(out));
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

}
