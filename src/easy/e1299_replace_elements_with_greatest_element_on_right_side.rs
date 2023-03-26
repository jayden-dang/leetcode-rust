#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut mx = -1;
        let mut result = vec![-1; arr.len()];
        for i in (0..arr.len()).rev() {
            result[i] = mx;
            if arr[i] > mx {
                mx = arr[i];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_elements() {
        assert_eq!(Solution::replace_elements(vec![1]), vec![-1]);
        assert_eq!(
            Solution::replace_elements(vec![1, 2, 3, 4, 5]),
            vec![5, 5, 5, 5, -1]
        );
        assert_eq!(
            Solution::replace_elements(vec![5, 4, 3, 2, 1]),
            vec![4, 3, 2, 1, -1]
        );
        assert_eq!(Solution::replace_elements(vec![]), vec![]);
    }
}
