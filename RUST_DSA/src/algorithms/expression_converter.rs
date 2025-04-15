use super::stack::Stack;
use std::collections::HashMap;

pub fn postfix_convert(expr: &str) -> Option<String> {
    let mut op_stack = Stack::new();
    let mut postfix_list = Vec::new();

    let mut src_str = Vec::new();
    for c in expr.chars() {
        src_str.push(c);
    }

    let mut operators = HashMap::new();
    operators.insert('(', 1);
    operators.insert(')', 1);

    operators.insert('+', 2);
    operators.insert('-', 2);

    operators.insert('*', 3);
    operators.insert('/', 3);

    for token in src_str {
        if token == '(' {
            op_stack.push(token);
        } else if (token >= 'A' && token <= 'Z') || (token >= '0' && token <= '9') {
            postfix_list.push(token);
        } else if token == ')' {
            let mut top = op_stack.pop().unwrap();
            while top != '(' {
                postfix_list.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            while (!op_stack.is_empty())
                && (operators[op_stack.peek().unwrap()] >= operators[&token])
            {
                postfix_list.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }

    while !op_stack.is_empty() {
        postfix_list.push(op_stack.pop().unwrap())
    }

    let mut postfix_str = "".to_string();
    for c in postfix_list {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    Some(postfix_str)
}
