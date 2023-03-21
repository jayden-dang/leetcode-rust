#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut ind: Vec<usize> = Vec::new();
        for i in 0..nums.len() {
            if nums[i] == 0 {
                ind.push(i);
            }
        }
        ind.reverse();
        for i in ind {
            nums.remove(i);
            nums.push(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0, 0, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0, 0, 0]);

        let mut nums = vec![1, 2, 3, 4, 5];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);

        let mut nums = vec![1, 0, 2, 0, 0, 3, 0, 4];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 0, 0, 0, 0]);
    }
}
