fn main() {
    println!("{}", is_palindrome(121));
    println!("{}", is_palindrome(-121));
}

pub fn is_palindrome(x: i32) -> bool {
    let mut temp = x;
    let mut y = 0;
    while temp > 0 {
        let rem = temp % 10;
        temp = (temp - rem) / 10;
        y = y*10 + rem;
    }
    x == y
}