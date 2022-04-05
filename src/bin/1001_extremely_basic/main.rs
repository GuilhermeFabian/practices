use std::io::{ stdin };

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
}

fn sum_two_integers(a: &i32, b: &i32) -> i64 {
    i64::from(*a) + i64::from(*b)
}

fn main() {
    let mut scan = Scanner::default();

    let a = scan.next::<i32>();
    let b = scan.next::<i32>();

    let x = sum_two_integers(&a, &b);

    println!("X = {}", x);
}


#[cfg(test)]
mod tests {
    use super::sum_two_integers;

    #[test]
    fn it_should_match_all_judge_output_samples() {
        let mut a = 10;
        let mut b = 9;

        let mut expected = 19;
        let mut received = sum_two_integers(&a, &b);

        assert_eq!(expected, received);

        a = -10;
        b = 4;

        expected = -6;
        received = sum_two_integers(&a, &b);

        assert_eq!(expected, received);

        a = 15;
        b = -7;

        expected = 8;
        received = sum_two_integers(&a, &b);

        assert_eq!(expected, received);
    }

    #[test]
    fn it_should_return_the_sum_correctly_when_both_numbers_reach_the_i32_limits() {
        let maximum_of_i32 = i32::MAX;
        let minimum_of_i32 = i32::MIN;

        let maximum_expected: i64 = 4294967294;
        let minimum_expected: i64 = -4294967296;

        let mut received = sum_two_integers(&maximum_of_i32, &maximum_of_i32);
        assert_eq!(received, maximum_expected);

        received = sum_two_integers(&minimum_of_i32, &minimum_of_i32);
        assert_eq!(received, minimum_expected);
    }

    #[test]
    fn it_should_return_a_negative_number_when_the_modulus_of_a_negative_number_is_greater_than_the_positive_number() {
        let a = -10;
        let b = 4;

        let expected = -6;

        let received = sum_two_integers(&a, &b);
        assert_eq!(received, expected);
    }

    #[test]
    fn it_should_return_zero_when_the_modulus_of_a_negative_number_is_equal_to_the_positive_number() {
        let a = -10;
        let b = 10;

        let expected = 0;

        let received = sum_two_integers(&a, &b);
        assert_eq!(received, expected);
    }
}
