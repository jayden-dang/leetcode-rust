#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let arr_copy = arr.clone();
        let new: Vec<i32> = arr.iter().map(|i| i * 2).collect();
        for i in new {
            if arr_copy.contains(&i) {
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
    #[should_panic]
    fn test_case_panic() {
        let arr = vec![3, 1, 7, 11];
        assert!(Solution::check_if_exist(arr));
    }

    #[test]
    fn test_case() {
        let arr1 = vec![-10, 12, -20, -8, 15];
        assert!(Solution::check_if_exist(arr1));
    }

    #[test]
    fn test_zero_array() {
        let arr = vec![0, 0];
        assert!(Solution::check_if_exist(arr));
    }

    #[test]
    fn test_random() {
        let arr = vec![-2, 0, 10, -19, 4, 6, -8];
        assert!(Solution::check_if_exist(arr));
    }
}
