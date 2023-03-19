#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut result = 0;
        nums.iter().for_each(|i| {
            if i != &val {
                result += 1;
            }
        });
        result
    }

    pub fn remove_element_sol_2(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let expected = 2;
        assert_eq!(Solution::remove_element(&mut nums, val), expected);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let expected = 5;
        assert_eq!(Solution::remove_element_sol_2(&mut nums, val), expected);
        assert_eq!(nums[..expected as usize], [0, 1, 3, 0, 4]);
    }
}
