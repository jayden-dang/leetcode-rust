#![allow(dead_code)]
struct Solution;

impl Solution {
    // fast runtime but big memory
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        (1..arr.len())
            .find(|&i| arr[i - 1] >= arr[i])
            .map_or(false, |j| {
                j > 1 && (j..arr.len()).all(|j| arr[j - 1] > arr[j])
            })
    }

    pub fn valid_mountain_array_2(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }
        let mut i = 0;
        let mut j = n - 1;
        while i < n - 1 && arr[i] < arr[i + 1] {
            i += 1;
        }
        while j > 0 && arr[j - 1] > arr[j] {
            j -= 1;
        }
        i > 0 && j < n - 1 && i == j
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mountain_array() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
        assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
        assert!(!Solution::valid_mountain_array(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9
        ]));
        assert!(!Solution::valid_mountain_array(vec![
            9, 8, 7, 6, 5, 4, 3, 2, 1, 0
        ]));
    }

    #[test]
    fn test_valid_mountain_array_2() {
        assert!(!Solution::valid_mountain_array_2(vec![2, 1]));
        assert!(!Solution::valid_mountain_array_2(vec![3, 5, 5]));
        assert!(Solution::valid_mountain_array_2(vec![0, 3, 2, 1]));
        assert!(!Solution::valid_mountain_array_2(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9
        ]));
        assert!(!Solution::valid_mountain_array_2(vec![
            9, 8, 7, 6, 5, 4, 3, 2, 1, 0
        ]));
    }
}
