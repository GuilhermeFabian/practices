use std::{io::stdin, collections::HashMap};


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

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut bucket: HashMap<i32, i32> = HashMap::new();

        let mut difference;

        for (index, number) in numbers.iter().enumerate() {
            difference = target - number;

            if bucket.contains_key(&difference) {
                if let Some(complementary_index) = bucket.get(&difference) {
                    return vec![*complementary_index, index as i32];
                }
            }

                bucket.insert(*number, index as i32);
        }

        vec![]
    }
}

fn main() {
    let mut scan = Scanner::default();

    let target = scan.next::<i32>();
    let mut numbers: Vec<i32> = vec![];

    loop {
        if let number = scan.next::<i32>() {
            numbers.push(number);
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_should_match_all_judge_output_samples() {
        let mut numbers = vec![2, 7, 11, 15];
        let mut target = 9;

        let mut expected = [0, 1];
        let mut received = Solution::two_sum(numbers, target);

        assert_eq!(received, expected);

        numbers = vec![5, -9, 1, 2];
        target = -7;

        expected = [1,3];
        received = Solution::two_sum(numbers, target);

        assert_eq!(received, expected);
    }
}
