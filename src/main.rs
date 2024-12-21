use aepl::parser::parse_ops;
use aepl::types::{AeplStack, Literal, Op, Value};

fn main() {
    let mut stack: AeplStack = Vec::new();

    // Example input: "5 10 + 3 *"
    let input = "✖5➕2 2🎉3🫸hello🫷";
    let parse_result = parse_ops(input);
    match parse_result {
        Err(e) => {
            println!("Error parsing input: {}", e);
            return;
        }
        Ok((_, ops)) => {
            eval_ops(&mut stack, ops);
        }
    }

    for value in stack.iter().rev() {
        println!("{}", value);
    }
}

fn eval_ops(stack: &mut AeplStack, ops: Vec<Op>) {
    let builtins = aepl::builtins::get_builtins();
    for op in ops.into_iter().rev() {
        match op {
            Op::Literal(literal) => stack.push(Value::Literal(literal)),
            Op::Identifier(identifier) => {
                if let Some(builtin) = builtins.get(&identifier) {
                    builtin(stack);
                } else {
                    println!("Unknown identifier: {}", identifier);
                }
            }
        }
    }
}
