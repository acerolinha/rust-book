# To panic or not to panic

## Panic is a MUST when:
1. Your code would end up in a bad state
2. Your code would cause undefined behavior
3. Your code would cause a security vulnerability
4. Your code is not memory safe
5. A failure is not an option
6. You can't rely on the type system

## Panic is not a MUST when:
1. You're writing an example
2. You're prototyping
3. You're writing a test
4. You're sure that the Result will never be an Err
5. A failure is an option