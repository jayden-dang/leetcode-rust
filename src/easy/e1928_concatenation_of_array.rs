#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let num = nums.clone();
        nums.extend(num.iter());
        nums
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_concatenation() {
        assert_eq!(Solution::get_concatenation(vec![]), vec![]);
        assert_eq!(Solution::get_concatenation(vec![1, 2, 3]), vec![1, 2, 3, 1, 2, 3]);
        assert_eq!(Solution::get_concatenation(vec![0]), vec![0, 0]);
        assert_eq!(Solution::get_concatenation(vec![-1, 5, 0]), vec![-1, 5, 0, -1, 5, 0]);
    }
}
