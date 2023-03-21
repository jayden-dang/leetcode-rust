#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut new = heights.clone();
        new.sort();
        let mut result = 0;
        for i in 0..heights.len() {
            if heights[i] != new[i] {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_checker() {
        // Test with an empty vector
        let heights: Vec<i32> = vec![];
        assert_eq!(Solution::height_checker(heights), 0);

        // Test with a vector containing only one element
        let heights = vec![5];
        assert_eq!(Solution::height_checker(heights), 0);

        // Test with a vector containing sorted elements
        let heights = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::height_checker(heights), 0);

        // Test with a vector containing reverse sorted elements
        let heights = vec![5, 4, 3, 2, 1];
        assert_eq!(Solution::height_checker(heights), 4);

        // Test with a vector containing partially sorted elements
        let heights = vec![1, 3, 2, 5, 4];
        assert_eq!(Solution::height_checker(heights), 4);

        // Test with a vector containing duplicate elements
        // [1 ,2, 2 , 3,3]
        let heights = vec![3, 3, 2, 2, 1];
        assert_eq!(Solution::height_checker(heights), 4);
    }
}
