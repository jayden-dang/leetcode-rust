use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        if set.len() < 3 {
            return *set.iter().max().unwrap();
        }
        let mut first = i32::MIN;
        let mut second = i32::MIN;
        let mut third = i32::MIN;
        for num in set {
            if num > first {
                third = second;
                second = first;
                first = num;
            } else if num > second {
                third = second;
                second = num;
            } else if num > third {
                third = num;
            }
        }
        third
    }

    pub fn third_max_sol_2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.dedup();
        nums[nums.len() - if nums.len() > 2 { 3 } else { 1 }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2, -2147483648]), -2147483648);
        assert_eq!(Solution::third_max(vec![1, 2, 2, 3]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![1]), 1);
    }

    #[test]
    fn test_third_max_sol_2() {
        assert_eq!(Solution::third_max_sol_2(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max_sol_2(vec![1, 2, -2147483648]), -2147483648);
        assert_eq!(Solution::third_max_sol_2(vec![1, 2, 2, 3]), 1);
        assert_eq!(Solution::third_max_sol_2(vec![1, 2]), 2);
        assert_eq!(Solution::third_max_sol_2(vec![1]), 1);
    }
}
