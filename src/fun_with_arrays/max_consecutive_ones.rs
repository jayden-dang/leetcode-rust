#![allow(dead_code)]
struct Solution;

impl Solution {
    fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut current_ones = 0;
        let mut max_ones = 0;
        for i in nums {
            if i == 1 {
                current_ones += 1;
            } else {
                current_ones = 0;
            }

            if current_ones > max_ones {
                max_ones = current_ones;
            }
        }

        max_ones
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn finded_3_element() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }

    #[test]
    fn finded_2_element() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 0]),
            2
        );
    }

    #[test]
    fn finded_1_element() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![0, 1, 0, 0, 1, 0]),
            1
        );
    }
}
