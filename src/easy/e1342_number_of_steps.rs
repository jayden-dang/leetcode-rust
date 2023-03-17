#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn number_of_steps(mut n: i32) -> i32 {
        let mut steps = 0;
        while n > 0 {
            if n % 2 == 0 {
                steps += 1;
                n /= 2;
            } else {
                steps += 1;
                n -= 1;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn steps_of_14() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    fn steps_of_8() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    fn steps_of_100() {
        assert_eq!(Solution::number_of_steps(100), 9);
    }
}
