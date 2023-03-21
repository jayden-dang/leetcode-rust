#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // Best Cases => O(1)
        if nums.is_empty() {
            return 0;
        }

        // Worst Cases
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                vec.push(i + 1);
            }
        }
        vec.reverse();
        for i in vec {
            nums.remove(i);
        }
        nums.len() as i32
    }

    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        // Best Cases => O(1)
        if nums.is_empty() {
            return 0;
        }

        // Worst Cases
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        // Test case 1: empty vector
        let mut nums1: Vec<i32> = Vec::new();
        assert_eq!(Solution::remove_duplicates(&mut nums1), 0);
        assert_eq!(nums1, []);

        // Test case 2: vector with no duplicates
        let mut nums2 = vec![1, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums2), 3);
        assert_eq!(nums2, [1, 2, 3]);

        // Test case 3: vector with duplicates
        let mut nums3 = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums3), 3);
        assert_eq!(nums3, [1, 2, 3]);

        // Test case 4: vector with all duplicates
        let mut nums4 = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::remove_duplicates(&mut nums4), 1);
        assert_eq!(nums4, [1]);

        // Test case 5: vector with some duplicates at beginning and end
        let mut nums5 = vec![1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums5), 3);
        assert_eq!(nums5, [1, 2, 3]);
    }

    #[test]
    fn test_remove_duplicates_2() {
        // Test case 1: empty vector
        let mut nums1: Vec<i32> = Vec::new();
        assert_eq!(Solution::remove_duplicates_2(&mut nums1), 0);
        assert_eq!(nums1, []);

        // Test case 2: vector with no duplicates
        let mut nums2 = vec![1, 2, 3];
        assert_eq!(Solution::remove_duplicates_2(&mut nums2), 3);
        assert_eq!(nums2, [1, 2, 3]);

        // Test case 3: vector with duplicates
        let mut nums3 = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates_2(&mut nums3), 3);
        assert_eq!(nums3, [1, 2, 3]);

        // Test case 4: vector with all duplicates
        let mut nums4 = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::remove_duplicates_2(&mut nums4), 1);
        assert_eq!(nums4, [1]);

        // Test case 5: vector with some duplicates at beginning and end
        let mut nums5 = vec![1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates_2(&mut nums5), 3);
        assert_eq!(nums5, [1, 2, 3]);
    }
}
