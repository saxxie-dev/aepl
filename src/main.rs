fn main() {
    let mut stack: Vec<i32> = Vec::new();

    // Example input: "5 10 + 3 *"
    let input = "✖ 5 ➕ -1 2";

    for token in input.split_whitespace().rev() {
        match token {
            "✖" => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                }
            }
            "➕" => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                }
            }
            "➖" => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(b - a);
                }
            }
            "🍾" => {
                if stack.len() >= 1 {
                    stack.pop();
                }
            }
            num => {
                if let Ok(n) = num.parse::<i32>() {
                    stack.push(n);
                }
            }
        }
    }

    println!("Final stack: {:?}", stack);
}

#[derive(Debug, Clone)]
enum StackValue {
    Integer(i32),
    Function { name: String, arity: usize },
}
