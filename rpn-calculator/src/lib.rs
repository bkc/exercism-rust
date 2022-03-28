#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for element in inputs.iter() {
        if let CalculatorInput::Value(x) = element {
            stack.push(*x);
            continue;
        }
        if stack.len() < 2 {
            return None;
        }
        let operand_b = stack.pop().unwrap();
        let operand_a = stack.pop().unwrap();

        match element {
            CalculatorInput::Add => {
                stack.push(operand_a + operand_b);
            }
            CalculatorInput::Subtract => {
                stack.push(operand_a - operand_b);
            }
            CalculatorInput::Multiply => {
                stack.push(operand_a * operand_b);
            }
            CalculatorInput::Divide => {
                stack.push(operand_a / operand_b);
            }
            _ => {
                panic!("unexpected branch")
            }
        }
    }
    if stack.len() == 1 {
        return Some(stack[0]);
    }
    None
}
