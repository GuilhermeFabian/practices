use std::io::stdin;

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

fn compute_circle_area(r: &f64) -> f64 {
    const PI: f64 = 3.14159;

    r.powf(2.0) * PI
}

fn main() {
    let mut scan = Scanner::default();
    let r = scan.next::<f64>();

    let a = compute_circle_area(&r);

    println!("A={:.4}", a);
}

#[cfg(test)]
mod tests {
    use super::compute_circle_area;

    #[test]
    fn it_should_match_all_judge_output_samples() {
        let mut r = 2.0;

        let mut expected = 12.5664;
        let mut received: f64 = compute_circle_area(&r);
        received = format!("{:.4}", received).parse::<f64>().unwrap();

        assert_eq!(expected, received);

        r = 100.64;

        expected = 31819.3103;
        received = compute_circle_area(&r);
        received = format!("{:.4}", received).parse::<f64>().unwrap();

        assert_eq!(expected, received);

        r = 150.00;

        expected = 70685.7750;
        received = compute_circle_area(&r);
        received = format!("{:.4}", received).parse::<f64>().unwrap();

        assert_eq!(expected, received);
    }
}
