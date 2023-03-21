#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut left = Vec::with_capacity(nums.len());
        let mut right = Vec::with_capacity(nums.len());
        nums.iter().for_each(|i| match i % 2 == 0 {
            true => left.push(*i),
            false => right.push(*i),
        });

        left.extend(right);
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        // Test with an empty vector
        let nums: Vec<i32> = vec![];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![]);

        // Test with a vector containing only even numbers
        let nums = vec![2, 4, 6, 8];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![2, 4, 6, 8]);

        // Test with a vector containing only odd numbers
        let nums = vec![1, 3, 5, 7];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![1, 3, 5, 7]);

        // Test with a vector containing both even and odd numbers
        let nums = vec![2, 1, 4, 3, 6, 5];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![2, 4, 6, 1, 3, 5]);

        // Test with a vector containing negative numbers
        let nums = vec![-2, 1, 0, -3, 6, -5];
        assert_eq!(
            Solution::sort_array_by_parity(nums),
            vec![-2, 0, 6, 1, -3, -5]
        );
    }
}
