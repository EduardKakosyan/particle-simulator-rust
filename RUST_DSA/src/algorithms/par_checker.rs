use super::stack::Stack;

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

pub fn par_checker(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c)
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            }

            let top = stack.pop().unwrap();
            if !par_match(top, c) {
                balance = false;
            }
        }

        index += 1;
    }

    balance && stack.is_empty()
}
