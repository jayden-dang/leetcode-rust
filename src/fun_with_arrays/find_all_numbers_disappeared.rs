#![allow(dead_code)]
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len() as i32;
        let mut result = HashSet::new();

        for n in nums {
            result.insert(n);
        }
        (1..=len).filter(|v| !result.contains(v)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let expected = vec![5, 6];
        assert_eq!(Solution::find_disappeared_numbers(nums), expected);

        let nums = vec![1, 1, 2, 2, 3, 3, 4, 5];
        let expected = vec![6, 7, 8];
        assert_eq!(Solution::find_disappeared_numbers(nums), expected);

        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = Vec::<i32>::new();
        assert_eq!(Solution::find_disappeared_numbers(nums), expected);
    }
}
