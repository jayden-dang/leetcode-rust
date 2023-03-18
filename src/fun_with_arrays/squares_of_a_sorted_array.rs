#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        let mut new: Vec<i32> = nums.iter().map(|i| i * i).collect();
        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            for i in 0..new.len() - 1 {
                if new[i] > new[i + 1] {
                    new.swap(i, i + 1);
                    is_sorted = false;
                }
            }
        }
        new
    }

    pub fn sorted_squares_sol_2(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        nums.sort_by_key(|a| a.abs());
        nums.iter().map(|x| x.pow(2)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares_empty_array() {
        assert_eq!(Solution::sorted_squares(vec![]), vec![]);
        assert_eq!(Solution::sorted_squares_sol_2(vec![]), vec![]);
    }

    #[test]
    fn test_sorted_squares_single_element_array() {
        assert_eq!(Solution::sorted_squares(vec![5]), vec![25]);
        assert_eq!(Solution::sorted_squares_sol_2(vec![5]), vec![25]);
    }

    #[test]
    fn test_sorted_squares_all_negative_array() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -3, -2, -1]),
            vec![1, 4, 9, 16]
        );
        assert_eq!(
            Solution::sorted_squares_sol_2(vec![-4, -3, -2, -1]),
            vec![1, 4, 9, 16]
        );
    }

    #[test]
    fn test_sorted_squares_all_positive_array() {
        assert_eq!(
            Solution::sorted_squares(vec![1, 2, 3, 4]),
            vec![1, 4, 9, 16]
        );
        assert_eq!(
            Solution::sorted_squares_sol_2(vec![1, 2, 3, 4]),
            vec![1, 4, 9, 16]
        );
    }

    #[test]
    fn test_sorted_squares_mixed_array() {
        assert_eq!(
            Solution::sorted_squares(vec![-3, -2, 1, 4]),
            vec![1, 4, 9, 16]
        );
        assert_eq!(
            Solution::sorted_squares_sol_2(vec![-3, -2, 1, 4]),
            vec![1, 4, 9, 16]
        );
    }
}
