# Practices

## Code Template

```rust
use std::{io::stdin};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
}

fn main() {
}

#[cfg(test)]
mod tests {
  use super::Solution;
}
```

## Running a Solution

```bash
cargo run -q 2>/dev/null --bin <bin_name>
```

## Running the Tests of a Solution

### All Tests

```bash
cargo test -q 2>/dev/null --bin <bin_name>
```

### A Specific Test

```bash
cargo test -q 2>/dev/null --bin <bin_name> <test_name>
```
