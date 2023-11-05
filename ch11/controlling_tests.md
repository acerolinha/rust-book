# Controlling How Tests Are Run

## To prevent tests of running in parallel
Use the `--test-threads=1` flag:

```bash
cargo test -- --test-threads=1
```

## To show output of successful tests
Use the `--show-output` flag:
    
```bash
cargo test -- --show-output
```

## To run a single test:

```bash
cargo test <test_name>
```

## To run multiple tests by filtering:

```bash
cargo test <filter>
```

### Example
Given the tests below, we can run all tests that contain `add` in their name by running:

```bash
cargo test add
```

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

## To ignore some test
Add the #[ignore] attribute:

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

## To run ignored tests:

```bash
cargo test -- --ignored
```
