//https://github.com/actix/actix-web/blob/master/actix-web/tests/test_httpserver.rs#L48
// likely need to follow this model where they spawn a thread for the webserver then interact with a rest client in another thread.
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[cfg(test)]
mod tests {
    use super::*;
    // test server start
    // test sending a request to the server /hello
    // test sending a request to the server /hello/<name>
    #[test]
    fn start_server() {
        let mut cmd = Command::cargo_bin("web").unwrap();
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("starting HTTP server at http://127.0.0.1:8080"));
    }
}
