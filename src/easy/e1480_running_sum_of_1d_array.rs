#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |prev, curr| {
                *prev += curr;
                Some(*prev)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::running_sum(nums), vec![1, 3, 6, 10]);
    }

    #[test]
    fn case_2() {
        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::running_sum(nums), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_3() {
        let nums = vec![3, 1, 2, 10, 1];
        assert_eq!(Solution::running_sum(nums), vec![3, 4, 6, 16, 17]);
    }
}
