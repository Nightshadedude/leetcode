fn main() {
    let s = "()[]{}".to_string();
    println!("{}", is_valid(s));
    let s = "(]".to_string();
    println!("{}", is_valid(s));
}

pub fn is_valid(s: String) -> bool {
    let parens = s.chars().collect::<Vec<char>>();
    let mut waiting = vec![]; 
    for p in parens.iter() {
        if waiting.len() == 0 {
            waiting.push(p.clone());
        } else if is_opening(*p) {
            waiting.push(p.clone());           
        } else if waiting.len() == 0 {
             return false;
        } else if is_valid_closing(*waiting.last().unwrap(), *p) {
            waiting.pop();
        } else {
            return false;
        }
    }
    if waiting.len() == 0 { return true; }
    return false;
}

pub fn is_opening(s: char) -> bool {
    if s == '{' { return true; }
    if s == '(' { return true; }
    if s == '[' { return true; }
    return false;
}

pub fn is_valid_closing(left: char, right: char) -> bool {
    if left == '{' && right == '}' { return true; }
    if left == '(' && right == ')' { return true; }
    if left == '[' && right == ']' { return true; }
    return false;
}