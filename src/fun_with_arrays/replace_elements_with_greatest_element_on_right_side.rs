struct Solution;

impl Solution {
    pub fn replace_elements(arr: &mut Vec<i32>) -> Vec<i32> {
        let mut len = arr.len() - 1;
        while len > 0 {
            if arr[len - 1] > arr[len] {
                arr[len] = -1;
                len -= 1;
            } else {
                arr[len - 1] = arr[len];
                len -= 1;
            }
        }
        vec![]
    }
}
