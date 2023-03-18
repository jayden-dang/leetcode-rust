struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in nums {
            let mut digits = 0;
            let mut n = i;
            while n != 0 {
                n /= 10;
                digits += 1;
            }
            if digits % 2 == 0 {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_numbers() {
        // Test Case 1
        let nums1 = vec![12, 345, 2, 6, 7896];
        assert_eq!(Solution::find_numbers(nums1), 2);

        // Test Case 2
        let nums2 = vec![555, 901, 482, 1771];
        assert_eq!(Solution::find_numbers(nums2), 1);

        // Test Case 4
        let nums3 = vec![10, 100, 1000, 10000];
        assert_eq!(Solution::find_numbers(nums3), 2);

        // Test Case 5
        let nums4 = vec![-12, -345, -2, -6, -7896];
        assert_eq!(Solution::find_numbers(nums4), 2);
    }
}
