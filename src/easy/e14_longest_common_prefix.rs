#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let mut out: Vec<u8> = vec![];
        // find the shortest string in this vec and bring to new variable
        let w = match strs.iter().enumerate().min_by_key(|(_, s)| s.len()) {
            Some((i,_)) => strs.swap_remove(i),
            None => return Default::default(),
        };

        for (i,b) in w.as_bytes().iter().enumerate() {
            if strs.iter().all(|s| s.as_bytes().get(i) == Some(b)) {
                out.push(*b);
            } else {
                break;
            }
        }
        String::from_utf8(out).unwrap()
    }

    pub fn longest_common_prefix_2(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut prefix = strs[0].clone();
        for i in strs.iter() {
            while !i.starts_with(&prefix) {
                prefix.pop();
            }
        }
        prefix
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let input = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        assert_eq!(Solution::longest_common_prefix(input), "fl");

        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix(input), "");
    }

    #[test]
    fn test_longest_common_prefix_basic() {
        let input = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        assert_eq!(Solution::longest_common_prefix_2(input), "fl");

        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix_2(input), "");
    }
}
