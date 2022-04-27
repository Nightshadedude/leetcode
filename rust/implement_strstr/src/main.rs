fn main() {
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    println!("{}", str_str(haystack, needle));
    
    let haystack = "aaaaa".to_string();
    let needle = "bba".to_string();
    println!("{}", str_str(haystack, needle));
}


pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 { return 0; }
    if let Some(ii) = haystack.find(&needle){
        return ii as i32;
    } else {
        return -1;
    }
}
