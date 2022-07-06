use RustSym::expr::*;

fn main() {
    let a = Box::new(Expr::Var("a".to_string()));
    let b = Box::new(Expr::Var("b".to_string()));
    let c = Box::new(Expr::Add(Box::clone(&a), Box::clone(&b)));
    let d = Box::new(Expr::Mul(Box::clone(&a), Box::clone(&c)));
    let d = Expr::Div(Box::clone(&c), Box::clone(&d));
    
    println!("{}",d);

    let res = d.to_node().unwrap();
    println!("{:?}", res);
}