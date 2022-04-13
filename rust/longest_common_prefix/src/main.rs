fn main() {
    let vs = vec!["aaa".to_string(),"aa".to_string(),"aaa".to_string()];
    println!("{}", longest_common_prefix(vs));
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strs = strs;
    if strs.len() == 1 {
        let ret = strs[0].clone();
        return ret;
    } else {
        if strs[0] == "".to_string() { return "".to_string(); }
        let mut charlist = strs.pop().unwrap().chars().collect::<Vec<char>>();
        for st in strs.iter() {
            for (ii, ch) in st.chars().enumerate() {
                if  st.len() < charlist.len() {
                    charlist.drain(st.len()..);                    
                }
                if ii  >= charlist.len() || charlist.len() == 0 {
                    break;
                } else if ch == charlist[ii] { // do nothing
                } else {
                    charlist.drain(ii..);
                    break;
                }
            }
        }
        return charlist.into_iter().collect();
    }
}