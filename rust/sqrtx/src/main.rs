fn main() {
    println!("{}", my_sqrt(10));
}

pub fn my_sqrt(x: i32) -> i32 {
    (x as f64).sqrt() as i32
}