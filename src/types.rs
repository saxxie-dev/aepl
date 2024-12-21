use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Text(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(x) => write!(f, "{}", x),
            Literal::Text(t) => write!(f, "🫸{}🫷", t),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(lit) => write!(f, "{}", lit),
        }
    }
}

pub type AeplStack = Vec<Value>;

#[derive(Debug, PartialEq)]
pub enum Op {
    Literal(Literal),
    Identifier(String),
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Literal(lit) => write!(f, "{}", lit),
            Op::Identifier(id) => write!(f, "{}", id),
        }
    }
}

// impl Value {
//     // Helper method to create a Value from a literal
//     pub fn from_literal(lit: Literal) -> Self {
//         Value::Literal(lit)
//     }
// }

// impl AeplStack {
//     pub fn new() -> Self {
//         Vec::new()
//     }

//     pub fn push_value(&mut self, value: Value) {
//         self.push(value);
//     }

//     pub fn pop_value(&mut self) -> Option<Value> {
//         self.pop()
//     }
// }
