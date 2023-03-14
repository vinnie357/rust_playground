# rust_playground

Learning and playing with Rust Programming Language.

https://www.rust-lang.org/learn/get-started

## Install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## vscode plugin

https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

If your project isn't in the root you have to tell the language server where to find your files.

https://github.com/rust-lang/rust-analyzer/issues/2649

```bash
mkdir .vscode
# if empty
cat << EOF > .vscode/settings.json
{
    "rust-analyzer.linkedProjects": [
        "/workspaces/rust_playground/examples/cli/Cargo.toml",
        "/workspaces/rust_playground/examples/hello/Cargo.toml",
        "/workspaces/rust_playground/examples/tests/Cargo.toml",
        "/workspaces/rust_playground/examples/web/Cargo.toml"
    ]
}
EOF
# if existing settings.json
jq '."rust-analyzer.linkedProjects" +=
    [
        "/workspaces/rust_playground/examples/cli/Cargo.toml",
        "/workspaces/rust_playground/examples/hello/Cargo.toml",
        "/workspaces/rust_playground/examples/tests/Cargo.toml",
        "/workspaces/rust_playground/examples/web/Cargo.toml"
    ]' \
    config.json > output.json

```

## .gitignore

https://github.com/github/gitignore/blob/main/Rust.gitignore

## Initialize a new project

```bash
cargo new hello-rust
```

## Run

```bash
cargo run
```

## Add crates

```bash
#cargo add <name>
cargo add clap
```

## Build Binary

```bash
#rustc
#cargo rustc
cargo build
```

## build Binary without cargo helpers

rustc src/main.rs

## Learn

These docs will be the basis for the examples below:

https://doc.rust-lang.org/book/

https://www.rust-lang.org/learn

https://github.com/rust-lang/rustlings/

https://doc.rust-lang.org/stable/rust-by-example/

## Examples

```bash
├── examples
│   ├── cli
│   ├── hello
│   ├── tests
│   └── web
```

## Testing

https://doc.rust-lang.org/book/ch11-00-testing.html

Trying to start all my code here with the tests and not just the examples.

### Folder structure

https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-directory

"unit tests go in the same files as the code"

In the examples this is **src/lib.rs**, but in their guides its **src/main.rs**

It looks like this is tied to the intent of the module, so if you do **cargo new <myapp>** you get **main.rs**
and if you do **cargo new <myapp> --lib** you get **lib.rs** for a library.

```
<module name>
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

### Rust uses annotations or decorators to tell the complier which code is for testing

```rust
// config for a test
#[cfg(test)]
// test function blocks
#[test]
// negative ( expect to fail)
#[should_panic]
```

### Runing the tests

```bash
#test all
cargo test
# test with args
cargo test -- --show-output
# test one assumes test/integration_test.rs exists
cargo test --test integration_test

```

### Testing notes

You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
