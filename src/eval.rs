use ast::{OpCode, AST};
use context::{Context};

pub fn run_ast(ctx: &Context, ast: &AST) {
    match ast.opcode {
        OpCode::Block => {
            run_ast_children(&ctx, &ast);
        },
        OpCode::Print => {
            println!("{:?}", &ast.args);
        },
        OpCode::Exit => {
            println!("exit");
        }
    }
}

#[inline]
fn run_ast_children(ctx: &Context, ast: &AST) {
    for a in &ast.chilren {
        run_ast(&ctx, &a);
    }
}
