#![allow(dead_code)]
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_freq = HashMap::new();
        let mut t_freq = HashMap::new();

        for c in s.chars() {
            *s_freq.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            *t_freq.entry(c).or_insert(0) += 1;
        }

        for (i, freq) in s_freq {
            if freq != t_freq.get(&i).cloned().unwrap_or(0) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert!(Solution::is_anagram(
            String::from("anagram"),
            String::from("nagaram")
        ));
        assert!(!Solution::is_anagram(
            String::from("rat"),
            String::from("car")
        ));
        assert!(Solution::is_anagram(String::from("a"), String::from("a")));
        assert!(Solution::is_anagram(String::from(""), String::from("")));
        assert!(!Solution::is_anagram(String::from("ab"), String::from("a")));
    }
}
