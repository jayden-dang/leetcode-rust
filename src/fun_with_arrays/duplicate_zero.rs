#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut vec: Vec<usize> = Vec::new();
        for (i, &item) in arr.iter().enumerate() {
            if item == 0 {
                vec.push(i);
            }
        }

        vec.reverse();
        for &i in vec.iter() {
            arr.insert(i, 0);
            arr.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_zeros() {
        // Test case 1: basic example
        let mut arr1 = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut arr1);
        assert_eq!(arr1, vec![1, 0, 0, 2, 3, 0, 0, 4]);

        // Test case 2: no zeros
        let mut arr2 = vec![1, 2, 3, 4, 5];
        Solution::duplicate_zeros(&mut arr2);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]);

        // Test case 3: all zeros
        let mut arr3 = vec![0, 0, 0, 0, 0];
        Solution::duplicate_zeros(&mut arr3);
        assert_eq!(arr3, vec![0, 0, 0, 0, 0]);

        // Test case 4: zeros at beginning
        let mut arr4 = vec![0, 1, 2, 3];
        Solution::duplicate_zeros(&mut arr4);
        assert_eq!(arr4, vec![0, 0, 1, 2]);

        // Test case 5: zeros at end
        let mut arr5 = vec![1, 2, 3, 0];
        Solution::duplicate_zeros(&mut arr5);
        assert_eq!(arr5, vec![1, 2, 3, 0]);

        // Test case 6: single zero
        let mut arr6 = vec![1, 0];
        Solution::duplicate_zeros(&mut arr6);
        assert_eq!(arr6, vec![1, 0]);

        // Test case 7: empty vector
        let mut arr7 = Vec::new();
        Solution::duplicate_zeros(&mut arr7);
        assert_eq!(arr7, Vec::new());
    }
}
