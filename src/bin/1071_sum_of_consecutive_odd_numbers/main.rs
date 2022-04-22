use std::{io::stdin};
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn try_next<T: std::str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok();
            }

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();

            if self.buffer.len() == 0 {
                break None;
            }
        }
    }
}

struct Solution {}

impl Solution {
    pub fn sum_consecutive_odd_numbers_between(x: &i32, y: &i32) -> i32 {
        let (lower_bound, upper_bound) = if *x < *y { (*x, *y) } else { (*y, *x) };
        let first_therm = Self::get_next_odd_number(&lower_bound);
        let last_therm = Self::get_previous_odd_number(&upper_bound);

        if first_therm > last_therm {
            return 0;
        }

        let count_odd_numbers = Self::count_odd_numbers_between(&first_therm, &last_therm);
        let sum_of_therms_in_arithmetic_progression = (first_therm + last_therm) * count_odd_numbers / 2;

        sum_of_therms_in_arithmetic_progression
    }

    fn get_next_odd_number(x: &i32) -> i32 {
        1 + 2 * f32::ceil(*x as f32 / 2.0) as i32
    }

    fn get_previous_odd_number(x: &i32) -> i32 {
        -1 + 2 * f32::floor(*x as f32 / 2.0) as i32
    }

    fn count_odd_numbers_between(lowest_value: &i32, highest_value: &i32) -> i32 {
        let rounded_down = f32::floor(*lowest_value as f32 / 2.0) as i32;
        let rounded_up = f32::ceil(*highest_value as f32 / 2.0) as i32;

        i32::abs(rounded_up - rounded_down)
    }
}

fn main() {
    let mut scan = Scanner::default();
    let x = scan.next::<i32>();
    let y = scan.next::<i32>();

    let answer = Solution::sum_consecutive_odd_numbers_between(&x, &y);

    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_should_match_all_judge_output_samples() {
        let mut x = 6;
        let mut y = -5;

        let mut expected = 5;
        let mut received = Solution::sum_consecutive_odd_numbers_between(&x, &y);

        assert_eq!(received, expected);

        x = 15;
        y = 12;

        expected = 13;
        received = Solution::sum_consecutive_odd_numbers_between(&x, &y);

        assert_eq!(received, expected);

        x = 12;
        y = 12;

        expected = 0;
        received = Solution::sum_consecutive_odd_numbers_between(&x, &y);

        assert_eq!(received, expected);

        x = 13;
        y = 15;

        expected = 0;
        received = Solution::sum_consecutive_odd_numbers_between(&x, &y);

        assert_eq!(received, expected);

        x = 14;
        y = 16;

        expected = 15;
        received = Solution::sum_consecutive_odd_numbers_between(&x, &y);

        assert_eq!(received, expected);
    }

    #[test]
    fn it_should_return_the_next_odd_number() {
        let mut x = 3;

        let mut expected = 5;
        let mut received = Solution::get_next_odd_number(&x);

        assert_eq!(received, expected);

        x = 4;

        expected = 5;
        received = Solution::get_next_odd_number(&x);

        assert_eq!(received, expected);

        x = 0;

        expected = 1;
        received = Solution::get_next_odd_number(&x);

        assert_eq!(received, expected);

        x = -1;

        expected = 1;
        received = Solution::get_next_odd_number(&x);

        assert_eq!(received, expected);

        x = -3;

        expected = -1;
        received = Solution::get_next_odd_number(&x);

        assert_eq!(received, expected);
    }

    #[test]
    fn it_should_return_the_previous_odd_number() {
        let mut x = 5;

        let mut expected = 3;
        let mut received = Solution::get_previous_odd_number(&x);

        assert_eq!(received, expected);

        x = 4;

        expected = 3;
        received = Solution::get_previous_odd_number(&x);

        assert_eq!(received, expected);

        x = 0;

        expected = -1;
        received = Solution::get_previous_odd_number(&x);

        assert_eq!(received, expected);

        x = 1;

        expected = -1;
        received = Solution::get_previous_odd_number(&x);

        assert_eq!(received, expected);

        x = -3;

        expected = -5;
        received = Solution::get_previous_odd_number(&x);

        assert_eq!(received, expected);
    }

    #[test]
    fn it_should_return_the_number_of_odd_elements_between_a_lower_bound_and_upper_bound() {
        let mut lowest_value = 1;
        let mut highest_value = 3;

        let mut expected = 2;
        let mut received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = 1;
        highest_value = 2;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = 2;
        highest_value = 4;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = 0;
        highest_value = 1;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = 0;
        highest_value = 2;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -1;
        highest_value = 1;

        expected = 2;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -2;
        highest_value = 2;

        expected = 2;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -1;
        highest_value = 0;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -2;
        highest_value = 0;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -3;
        highest_value = -1;

        expected = 2;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -2;
        highest_value = -1;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -4;
        highest_value = -2;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = 0;
        highest_value = 0;

        expected = 0;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = 1;
        highest_value = 1;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = 2;
        highest_value = 2;

        expected = 0;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -1;
        highest_value = -1;

        expected = 1;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);

        lowest_value = -2;
        highest_value = -2;

        expected = 0;
        received = Solution::count_odd_numbers_between(&lowest_value, &highest_value);

        assert_eq!(received, expected);
    }
}
