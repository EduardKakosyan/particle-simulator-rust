use super::stack::Stack;
use std::collections::HashMap;

fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else if "/" == op {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!");
        }
        op1 / op2
    } else {
        panic!("OperationError: Invalid operator {:?}", op);
    }
}

pub fn postfix_convert(expr: &str) -> Option<i32> {
    if expr.len() < 5 {
        return None;
    }

    let mut ops = Stack::new();
    for token in expr.split_whitespace() {
        if ("0".."9").contains(&token) {
            ops.push(token.parse::<i32>().unwrap())
        } else {
            let op2 = ops.pop().unwrap();
            let op1 = ops.pop().unwrap();
            let res = do_calc(token, op1, op2);
            ops.push(res);
        }
    }

    Some(ops.pop().unwrap())
}
