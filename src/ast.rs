#[derive(Debug)]
pub enum OpCode {
    Block,
    Print,
    Exit
}

#[derive(Debug)]
pub enum Value {
    None,
    Number(f64),
    String(String),
    Function(String),
    Block(AST),
}

#[derive(Debug)]
pub struct AST {
    pub opcode: OpCode,
    pub args: Vec<Value>,
    pub chilren: Vec<AST>,
}

impl AST {
    pub fn new(opcode: OpCode, args: Option<Vec<Value>>) -> AST {
        AST {
            opcode: opcode,
            args: match args {
                None => vec![],
                Some(a) => a,
            },
            chilren: vec![]
        }
    }

    #[inline]
    pub fn push(&mut self, ast: AST) {
        self.chilren.push(ast);
    }
}
