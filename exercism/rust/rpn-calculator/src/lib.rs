#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for arg in inputs.iter() {
        match arg {
            &CalculatorInput::Value(v) => {
                stack.push(v);

            },
            _ => {
                if stack.len() > 1 {
                    process_arg(&mut stack, arg)
                }
                else {
                    stack.pop();
                }
            },
        }
    }
    if stack.len() > 1 {
        return None;
    }
    return stack.pop();
}

fn process_arg(stack: &mut Vec<i32>, arg: &CalculatorInput) {
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    match arg {
        CalculatorInput::Add => {
            stack.push(left + right);
        },
        CalculatorInput::Divide => {
            stack.push(left / right);
        },
        CalculatorInput::Subtract => {
            stack.push(left - right);
        },
        CalculatorInput::Multiply => {
            stack.push(left * right);
        }
        CalculatorInput::Value(_) => {
        }
    }
}
