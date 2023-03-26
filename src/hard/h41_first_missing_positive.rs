#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut present = vec![false; n + 1];

        for num in nums.iter() {
            if *num > 0 && *num <= n as i32 {
                present[*num as usize] = true;
            }
        }
        println!("{:?}", present);

        for i in 1..=n {
            if !present[i] {
                return i as i32;
            }
        }

        (n + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let nums = vec![1, 2, 0];
        assert_eq!(Solution::first_missing_positive(nums), 3);
    }

    #[test]
    fn test_two() {
        let nums = vec![3, 4, -1, 1];
        assert_eq!(Solution::first_missing_positive(nums), 2);
    }

    #[test]
    fn test_three() {
        let nums = vec![7, 8, 9, 11, 12];
        assert_eq!(Solution::first_missing_positive(nums), 1);
    }
}
