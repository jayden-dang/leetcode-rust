#![allow(dead_code)]
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let mut num_to_idx: HashMap<i32, usize> = HashMap::with_capacity(len);
        for (idx, val ) in nums.iter().enumerate() {
            let expected_num = target - val;
            match num_to_idx.get(&expected_num) {
                Some(&prev_idx) => {
                    return vec![prev_idx as i32, idx as i32];
                },
                None => {
                    num_to_idx.insert(*val, idx);
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        // Test case 1: Simple case with 2 numbers
        let nums1 = vec![2, 7];
        let target1 = 9;
        let result1 = Solution::two_sum(nums1, target1);
        assert_eq!(result1, vec![0, 1]);

        // Test case 2: Array contains negative numbers
        let nums2 = vec![2, -1, 5, -6];
        let target2 = -4;
        let result2 = Solution::two_sum(nums2, target2);
        assert_eq!(result2, vec![0, 3]);

        // Test case 3: Array contains duplicates
        let nums3 = vec![3, 2, 4, 3];
        let target3 = 6;
        let result3 = Solution::two_sum(nums3, target3);
        assert!(result3 == vec![0, 3] || result3 == vec![1, 2]);

        // Test case 4: Array contains large numbers
        let nums4 = vec![100000000, 200000000, 300000000, 400000000];
        let target4 = 500000000;
        let result4 = Solution::two_sum(nums4, target4);
        assert!(result4 == vec![0, 3] || result4 == vec![1,2]);

        // Test case 5: Empty array
        let nums5 = vec![];
        let target5 = 5;
        let result5 = Solution::two_sum(nums5, target5);
        assert_eq!(result5, vec![]);
    }
}
