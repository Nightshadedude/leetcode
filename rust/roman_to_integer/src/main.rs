fn main() {
    let s = "MCMXCIV".to_string();
    println!("{}", roman_to_int(s));
    let s = "I".to_string();
    println!("{}", roman_to_int(s));
    let s = "III".to_string();
    println!("{}", roman_to_int(s));
    let s = "IV".to_string();
    println!("{}", roman_to_int(s));
}


pub fn roman_to_int(s: String) -> i32 {
    let mut total = 0;
    let mut sp = s.split("").collect::<Vec<&str>>();
    let mut total = 0;
    let mut current = 0;
    let mut ii = 0;
    while ii < sp.len() {
        current = get_val(sp[ii]);
        if ii + 1 < sp.len() { 
            if get_val(sp[ii+1]) > current && current != 0 {
                total += get_val(sp[ii+1]) - current;
                ii += 1;
            } else {
                total += current;
            }
        } else {
            total += current;   
        }
        ii += 1;
    }
    if total == 0 { current }
    else { total }
}

fn get_val(s: &str) -> i32 {
    match s {
        "I" => 1,
        "V" => 5,
        "X" => 10,
        "L" => 50,
        "C" => 100,
        "D" => 500,
        "M" => 1000,
        _ => {
            0
        },
    }
}