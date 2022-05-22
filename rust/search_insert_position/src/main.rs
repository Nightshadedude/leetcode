pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i,num) in nums.iter().enumerate() {
        if num >= &target { return i as i32; }
    }
    nums.len() as i32
}


fn main() {
    let v = vec![1,3,5,6];
    println!("{}", search_insert(v, 5));
    let v = vec![1,3,5,6];
    println!("{}", search_insert(v, 2));
    let v = vec![1,3,5,6];
    println!("{}", search_insert(v, 7));
}
