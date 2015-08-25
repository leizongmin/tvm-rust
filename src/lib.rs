pub use ast::{OpCode, Value, AST};
pub use context::{Context};
pub use eval::{run_ast};

mod ast;
mod context;
mod eval;
