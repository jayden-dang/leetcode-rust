#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for i in 1..=n {
            match (i % 3, i % 5) {
                (0, 0) => result.push("FizzBuzz".into()),
                (0, _) => result.push("Fizz".into()),
                (_, 0) => result.push("Buzz".into()),
                _ => result.push(i.to_string()),
            };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn elements_3() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn element_5() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn element_15() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
