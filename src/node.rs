//use crate::functions::Func;

#[derive(Debug)]
pub enum Node {
    Num(i32),
    Var(String),
    Sum(Vec<Node>),
    Prod(Vec<Node>),
    Pow(Vec<Node>),
    Func(String, Vec<Node>)
}

//use colored::Colorize;
//use std::fmt::{self,Display};
// impl Display for Node {
//     fn fmt(&self, io:&mut fmt::Formatter)->fmt::Result {
//         use Node::*;
//         match self {
//             Var(s) => write!(io,"{}",s.bold().blue().italic()),
//             _ => Ok(()),
//         }
//     }
// }