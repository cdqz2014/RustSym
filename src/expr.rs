use std::fmt::{self, Display};
use colored::*;

use crate::node::Node;
//use crate::functions::Func;

#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Add(Box<Expr>,Box<Expr>),
    Sub(Box<Expr>,Box<Expr>),
    Mul(Box<Expr>,Box<Expr>),
    Div(Box<Expr>,Box<Expr>),
    Pow(Box<Expr>,Box<Expr>),
    Func(String, Box<Expr>)
}

impl Display for Expr {
    fn fmt(&self, io: &mut fmt::Formatter)->fmt::Result {
        use self::Expr::*;
        match self {
            Var(ex) => write!(io, "{}", ex.bold().blue().italic()),
            Add(x,y) => write!(io,"({}+{})",x,y),
            Sub(x,y) => write!(io,"({}-{})",x,y),
            Mul(x,y) => write!(io,"({})*({})",x,y),
            Div(x,y) => write!(io,"({})/({})",x,y),
            Pow(x,y) => write!(io,"{}^({})",x,y),
            Func(f,x) => write!(io,"{}({})",f,x)
        }
    }
}

#[derive(Debug)]
pub struct ExprError(String);
impl Display for ExprError {
    fn fmt(&self, io: &mut fmt::Formatter)->fmt::Result {
        write!(io,"function {} is not implemented", self.0.bold().red())
    }
}


impl Expr {
    pub fn to_node(&self)->Result<Node, ExprError> {
        // iterate match
        Ok(match self {
            Expr::Var(s) => Node::Var(s.clone()),
            Expr::Add(lhs, rhs) => Node::Sum(vec![lhs.to_node()?, rhs.to_node()?]),
            Expr::Sub(lhs, rhs) => Node::Sum(vec![lhs.to_node()?, Node::Prod(vec![Node::Num(-1), rhs.to_node()?])]),
            Expr::Mul(lhs, rhs) => Node::Prod(vec![lhs.to_node()?, rhs.to_node()?]),
            Expr::Div(lhs, rhs) => Node::Prod(vec![lhs.to_node()?, Node::Pow(vec![rhs.to_node()?, Node::Num(-1)])]),
            Expr::Pow(lhs, rhs) => Node::Pow(vec![lhs.to_node()?, rhs.to_node()?]),
            Expr::Func(f, arg) => {
                let func = match f.as_str() {
                    "sin" => "sin".to_string(),
                    "cos" => "cos".to_string(),
                    "tan" => "tan".to_string(),
                    "exp" => "exp".to_string(),
                    s => return Err(ExprError(s.to_string()))
                };
                Node::Func(func, vec![arg.to_node()?])
            }
        })
    }
}
