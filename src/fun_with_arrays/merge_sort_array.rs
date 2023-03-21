#![allow(dead_code, unused_variables)]
struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
        let mut new_vec = Vec::new();
        let mut left = 0;
        let mut right = 0;
        while left < m && right < n {
            if nums1[left as usize] < nums2[right as usize] {
                new_vec.push(nums1[left as usize]);
                left += 1;
            } else {
                new_vec.push(nums2[right as usize]);
                right += 1;
            }
        }

        while left < m {
            new_vec.push(nums1[left as usize]);
            left += 1;
        }

        while right < n {
            new_vec.push(nums2[right as usize]);
            right += 1;
        }
        new_vec
    }

    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //     // Best Cases => O(1)
    //     nums1.split_off(m as usize);
    //     nums1.extend(nums2.to_vec());
    //     nums1.sort();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        // Test case 1: empty vectors
        let mut nums1: Vec<i32> = Vec::new();
        let mut nums2: Vec<i32> = Vec::new();
        let expected = Vec::new();
        assert_eq!(Solution::merge(&mut nums1, 0, &mut nums2, 0), expected);

        // Test case 2: one empty vector, one non-empty vector
        let mut nums3 = vec![1, 2, 3];
        let mut nums4: Vec<i32> = Vec::new();
        let expected2 = vec![1, 2, 3];
        assert_eq!(Solution::merge(&mut nums3, 3, &mut nums4, 0), expected2);

        // Test case 3: two non-empty vectors with no overlap
        let mut nums5 = vec![1, 2, 3];
        let mut nums6 = vec![4, 5, 6];
        let expected3 = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::merge(&mut nums5, 3, &mut nums6, 3), expected3);

        // Test case 4: two non-empty vectors with overlap
        let mut nums7 = vec![1, 2, 4];
        let mut nums8 = vec![3, 5, 6];
        let expected4 = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::merge(&mut nums7, 3, &mut nums8, 3), expected4);

        // Test case 5: vectors of different lengths
        let mut nums9 = vec![1, 2, 3, 0, 0, 0];
        let mut nums10 = vec![2, 5, 6];
        let expected5 = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(Solution::merge(&mut nums9, 3, &mut nums10, 3), expected5);
    }
}
