# Rust Functional Programming Linting

This simple crate provides a single macro, `apply_deny_all`, that disallows all imperative constructs in the Rust programming language, effectively turning it into pure functional programming. Such constructs include the following keywords:

- Mutation: `mut`
- Loop: `for`, `while`, `loop`
- Loop Operations: `continue`, `break`

## Usage

Apply it to your entire program with the macro and avoid doing any of the things in the functions below.

```rs
#[rust_fp_linting::apply_deny_all]

fn foo(mut a: u32) {
    // This will give an error because of existence of `mut` keyword.
}

fn bar() {
    for v in (0..10) {
        // This will give an error because of `for` loop.
    }
}

fn baz() {
    while true {
        // This will give an error because of `while` loop.
    }

    loop {
        // This is syntactic sugar for the above, so the same thing will happen.
    }
}

fn main() {

}

```

## License

All code included in this repository is licensed under the terms of the [MIT License](LICENSE).
