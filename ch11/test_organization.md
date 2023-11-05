# Test Organization

## Unit Tests

- Should test each unit of code in isolation
- Should be put in the src directory, in each file with the code it tests
- Should be put in a `tests` module, with `#[cfg(test)]` attribute

## Integration Tests
- Should test whether many parts of the library work together correctly
- Should only call functions from the library's public API
- Should be put in the `src/tests` directory
- Doesn't need `#[cfg(test)]` attribute