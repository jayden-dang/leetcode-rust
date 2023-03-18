#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // Best Cases => O(1)
        nums1.split_off(m as usize);
        nums1.extend(nums2.to_vec());
        nums1.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        // Test case 1: basic test with both arrays having elements
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

        // Test case 2: nums1 is initially empty
        let mut nums1 = vec![];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);

        // Test case 3: nums2 is initially empty
        let mut nums1 = vec![1, 2, 3];
        let m = 3;
        let mut nums2 = vec![];
        let n = 0;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 3]);

        // Test case 4: nums1 and nums2 have same elements
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![1, 2, 3];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 1, 2, 2, 3, 3]);

        // Test case 5: nums1 and nums2 have no common elements
        let mut nums1 = vec![1, 2, 3];
        let m = 3;
        let mut nums2 = vec![4, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
