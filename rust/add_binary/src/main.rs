fn main() {
    println!("{}", add_binary("001".to_string(), "110".to_string()));
    println!("{}", add_binary("001".to_string(), "111".to_string()));
    println!("{}", add_binary("11111111111001".to_string(), "11011111111".to_string()));

}

pub fn add_binary(a: String, b: String) -> String {
    let ba = u128::from_str_radix(&a, 2).expect("Not a binary number!");
    let bb = u128::from_str_radix(&b, 2).expect("Not a binary number!");
    let bc = format!("{:b}", ba + bb);
    bc
}