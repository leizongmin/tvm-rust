extern crate tvm;
use tvm::*;

fn main() {
    let mut a = AST::new(OpCode::Block, None);
    a.push(AST::new(OpCode::Print, Some(vec![Value::String("hello".to_string())])));
    a.push(AST::new(OpCode::Exit, Some(vec![Value::Number(0.0)])));
    println!("{:?}", a);

    let mut c = Context::new();
    run_ast(&c, &a);
    println!("{:?}", c);
}
