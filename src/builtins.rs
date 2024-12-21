use crate::types::{AeplStack, Literal, Value};
use std::collections::HashMap;

type BuiltinFn = fn(&mut AeplStack);

pub fn get_builtins() -> HashMap<String, BuiltinFn> {
    let mut builtins = HashMap::new();

    builtins.insert("✖".to_string(), multiply as BuiltinFn);
    builtins.insert("➕".to_string(), add as BuiltinFn);
    builtins.insert("➖".to_string(), subtract as BuiltinFn);
    builtins.insert("🎉".to_string(), pop as BuiltinFn);

    builtins
}

fn multiply(stack: &mut AeplStack) {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        if let (Value::Literal(Literal::Int(a)), Value::Literal(Literal::Int(b))) = (a, b) {
            stack.push(Value::Literal(Literal::Int(a * b)));
        }
    }
}

fn add(stack: &mut AeplStack) {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        if let (Value::Literal(Literal::Int(a)), Value::Literal(Literal::Int(b))) = (a, b) {
            stack.push(Value::Literal(Literal::Int(a + b)));
        }
    }
}

fn subtract(stack: &mut AeplStack) {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        if let (Value::Literal(Literal::Int(a)), Value::Literal(Literal::Int(b))) = (a, b) {
            stack.push(Value::Literal(Literal::Int(a - b)));
        }
    }
}

fn pop(stack: &mut AeplStack) {
    stack.pop();
}
