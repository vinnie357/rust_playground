// https://rust-cli.github.io/book/tutorial/testing.html
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn show_help() {
        let mut cmd = Command::cargo_bin("cli").unwrap();
        cmd.arg("--help");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Simple program to greet a person"));
    }
    #[test]
    fn no_name() {
        let mut cmd = Command::cargo_bin("cli").unwrap();
        cmd.arg("--name");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("error: a value is required for '--name <NAME>' but none was supplied"));
    }
    #[test]
    fn use_name() {
        let mut cmd = Command::cargo_bin("cli").unwrap();
        cmd.arg("--name").arg("Declan");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Hello, Declan!"));
    }
    #[test]
    fn use_count() {
        // this isn't counting the number of times the pattern is repeated, so any match will do.
        let count_arg = "2";
        let count: usize = count_arg.parse().unwrap();
        let mut cmd = Command::cargo_bin("cli").unwrap();
        cmd.arg("--name").arg("Declan").arg("--count").arg(count_arg);
        // maybe tie the (n) count to the pattern times repeating for matching next?
        let mesage="Hello, Declan!\n".repeat(count as usize);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(mesage));
            // .stdout(predicate::str::contains("Hello, Declan!\nHello, Declan!"));
    }
}
