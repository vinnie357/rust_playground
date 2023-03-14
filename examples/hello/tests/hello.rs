// https://rust-cli.github.io/book/tutorial/testing.html
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_test() {
        let mut cmd = Command::cargo_bin("hello").unwrap();
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Hello, Declan!"));
    }
}
