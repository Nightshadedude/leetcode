fn main() {
    let mut v:Vec<i32> = vec![3,2,2,3];
    println!("{}", remove_element(&mut v, 3));
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut num_clone = nums.clone();
    let len = nums.len();
    num_clone.reverse();
    for (ii, elem) in num_clone.iter().enumerate() {
        if val == *elem {nums.remove(len-1-ii);}
    }
    println!("{:?}", nums);
    nums.len() as i32
}