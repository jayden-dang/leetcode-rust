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

    pub fn number_of_steps_2(n: i32) -> i32 {
        fn count_internal(num: i32, total_steps: i32) -> i32 {
            if num == 0 {
                total_steps
            } else if num % 2 == 0 {
                count_internal(num / 2, total_steps + 1)
            } else {
                count_internal(num - 1, total_steps + 1)
            }
        }
        count_internal(n, 0)
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

    #[test]
    fn steps_of_2_14() {
        assert_eq!(Solution::number_of_steps_2(14), 6);
    }

    #[test]
    fn steps_of_2_8() {
        assert_eq!(Solution::number_of_steps_2(8), 4);
    }

    #[test]
    fn steps_of_2_100() {
        assert_eq!(Solution::number_of_steps_2(100), 9);
    }
}
