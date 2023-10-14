use regex::Regex;


enum RegexError{
    NoMatch
}
enum MyError {
    RegexError(RegexError)
}

fn build_regex(template: &str) -> &str {
    let mut ans = "";
    for ch in template.chars() {
        match ch {
            '*' => {
                ans.push('(');
                ans.push('.');
                ans.push(ch);
                ans.push(')');
            }
            //TODO replace to smth like      ch in "\\[]()..."
            '\\' | '[' | ']' | '(' | ')' | '^' | '$' | '.' | '|' | '?' | '+' | '/' => {
                ans.push('\\');
                ans.push(ch);
            }
            _ => {
                ans.push(ch);
            }
        }
    }
    ans
}

fn select_data(template: &str, string: &str) -> Result<Vec<String>, MyError> {
    let mut ans = Vec::new();
    let re = Regex::new(build_regex(template))?;
    let Some(string_data) = re.captures(string) else {
        Err(MyError::RegexError(RegexError::NoMatch))
    };

    println!("{:?}", string_data);
    Ok(ans)
}


fn main() {
    let template = "path/to/some_*_filename.*";
    println!("{}",build_regex(template));
    // let file = "";
    // let data = select_data(a, "");
}
