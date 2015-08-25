use std::collections::HashMap;
use ast::{Value};

#[derive(Debug)]
pub struct Context {
    variables: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
        }
    }
}
