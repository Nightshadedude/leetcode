fn main() {
    let mut test: Vec<i32> = vec![1,1,2];
    println!("{}",remove_duplicates(&mut test));
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 1 || nums.len() == 0 { return nums.len() as i32; }
    let mut check = true;
    let mut ii = 0;
    while check {
        if ii < nums.len()-1 {
            if nums[ii] == nums[ii+1] {
                nums.remove(ii);
            } else {
                ii += 1;
            }
        } else {
            check  = false;
        }
    }
    nums.len() as i32
}