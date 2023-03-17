#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximun_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|i| i.iter().sum()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn array_2d() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(Solution::maximun_wealth(accounts), 6);
    }

    #[test]
    fn array_3x2() {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(Solution::maximun_wealth(accounts), 10);
    }

    #[test]
    fn array_3x3() {
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(Solution::maximun_wealth(accounts), 17);
    }
}
