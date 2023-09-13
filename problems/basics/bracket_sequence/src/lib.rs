#![forbid(unsafe_code)]

pub fn is_correct_bracket_sequence(s: &str) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        if ch == '(' || ch == '{' || ch == '[' {
            stack.push(ch);
        } else {
            let lst = stack.pop();
            if ch == ')' && lst == Some('(')
                || ch == ']' && lst == Some('[')
                || ch == '}' && lst == Some('{')
            {
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
