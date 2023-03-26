#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for num in nums {
            if let std::collections::hash_map::Entry::Occupied(_) = map.entry(num) {
                return true;
            }
            map.insert(num, ());
        }
        false
    }

    fn contains_duplicate_2(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for num in nums {
            if !set.insert(num) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 4, 5, 1]));
        assert!(Solution::contains_duplicate(vec![
            1, 1, 2, 2, 3, 3, 4, 4, 5, 5
        ]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4, 5]));
        assert!(!Solution::contains_duplicate(vec![1]));
        assert!(!Solution::contains_duplicate(vec![]));
    }

    #[test]
    fn test_contains_duplicate_2() {
        assert!(!Solution::contains_duplicate_2(vec![1, 2, 3, 4, 5]));
        assert!(Solution::contains_duplicate_2(vec![1, 2, 3, 4, 5, 1]));
        assert!(Solution::contains_duplicate_2(vec![
            1, 1, 2, 2, 3, 3, 4, 4, 5, 5
        ]));
        assert!(!Solution::contains_duplicate_2(vec![1]));
        assert!(!Solution::contains_duplicate_2(vec![]));
    }
}
