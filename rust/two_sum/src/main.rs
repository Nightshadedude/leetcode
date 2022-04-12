fn main() {
    let test = vec![2,7,11,15];
    println!("{:?}", two_sum(test, 9));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums;
    let mut found = vec![];
    while found.len() == 0 {
        let last = (nums.len()-1, nums.pop().unwrap());
        for (e,x) in nums.iter().enumerate(){
            if last.1 + x == target {
                found.push(e as i32);
                found.push(last.0 as i32);
            }
        }
    }
    found
}