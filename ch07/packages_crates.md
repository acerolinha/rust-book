# Packages and Crates

1. **crate**
    - is the smallest amount of code `rustc` considers at a time.
    - can be a binary or a library
         - **binary**: can be compiled to an executables
        - **library**: don't compile to an executable

2. **package**
    - is a bundle of one or more crates
    - can contain many binary crates
    - can contain only one library crate
    - must contain at least one crate