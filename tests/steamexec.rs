// //! Here we're using assert_cmd to test the command line interface of our main binary
// //! https://github.com/assert-rs/assert_cmd
// //! https://rust-lang-nursery.github.io/cli-wg/tutorial/testing.html
//
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn prints_the_help() {
    let mut command = Command::cargo_bin("boson").unwrap();
    command.args(&["-h"]).assert().success().stdout(
        "boson BOSON_VERSION
Alan Jenkins <alan.james.jenkins@gmail.com>
Manages launching games using Valve's Proton Linux compatibility layer and associated options.

USAGE:
    boson [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file\n",
    );
}

#[test]
fn config_file_path_does_not_exist() {
    let mut command = Command::cargo_bin("boson").unwrap();
    command
        .env_remove("RUST_BACKTRACE") // ensure that the whole backtrace doesn't get printed
        .args(&["--config", "/tmp/afdadlfaldfladsfadf.toml"]) // provide a non-integer argument
        .assert() // assert() function provided by the `OutputAssertExt` trait
        .failure()
        .code(1)
        .stderr("Unable to read the config file: No such file or directory (os error 2)\n");
}

#[test]
fn config_file_path_is_not_a_file() {
    let mut command = Command::cargo_bin("boson").unwrap();
    command
        .env_remove("RUST_BACKTRACE") // ensure that the whole backtrace doesn't get printed
        .args(&["--config", "/tmp/"]) // provide a non-integer argument
        .assert() // assert() function provided by the `OutputAssertExt` trait
        .failure()
        .code(1)
        .stderr("Unable to read the config file: Is a directory (os error 21)\n");
}

#[test]
fn invalid_config_file_contents_unterminated_quote() {
    let config_dir = TempDir::new("boson_test").unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut config_file_handle = File::create(&config_path).unwrap();

    match config_file_handle.write_all(b"config = \"") {
        Err(err) => panic!("{:?}", err),
        Ok(()) => {}
    };

    let mut command = Command::cargo_bin("boson").unwrap();
    command
        .env_remove("RUST_BACKTRACE") // ensure that the whole backtrace doesn't get printed
        .args(&["--config", config_path.to_str().unwrap()]) // provide a non-integer argument
        .assert() // assert() function provided by the `OutputAssertExt` trait
        .failure()
        .code(1)
        .stderr("Unable to parse the config file: unterminated string at line 1 column 10\n");
}
