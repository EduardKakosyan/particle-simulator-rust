use super::stack::Stack;

pub fn par_checker1(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            }
            let _r = stack.pop();
        }

        index += 1;
    }

    balance && stack.is_empty()
}

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

pub fn par_checker2(par: &str) -> bool {
    let mut char_list = Vec::new();

    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        } else {
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

pub fn par_checker3(par: &str) -> bool {
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
