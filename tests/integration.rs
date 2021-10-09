use std::process::Command;

const BASE_HELP_INFO: &str = "tio 0.1.0
Matthew Maxwell <maxwellmattryan@gmail.com>
CLI tool for broadcasting and searching messages on the IOTA Tangle

USAGE:
    tio <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    broadcast    Broadcast a message to the IOTA Tangle
    help         Prints this message or the help of the given subcommand(s)
    search       Search for a message on the IOTA Tangle
";

const INVALID_COMMAND: &str =
    "error: Found argument 'bad-command' which wasn't expected, or isn't valid in this context

USAGE:
    tio <SUBCOMMAND>

For more information try --help
";

const INVALID_SEARCH_ARGS: &str = "error: The following required arguments were not provided:
    <hash>

USAGE:
    tio search [OPTIONS] <hash>

For more information try --help
";

const BASE_BROADCAST_OUTPUT: &str = "CONTENT: \"TIO_MESSAGE\"
INDEX: \"TIO_DATA\"
SIZE: 11 byte(s)
";

const OTHER_BROADCAST_OUTPUT: &str = "CONTENT: \"THIS IS FOR AN INTEGRATION TEST\"
INDEX: \"TIO INTEGRATION TEST DATA INDEX\"
SIZE: 31 byte(s)
";

const BASE_SEARCH_OUTPUT: &str = "CONTENT: \"THIS IS FOR AN INTEGRATION TEST\"
SIZE: 31 byte(s)
";

mod integration {
    use super::*;

    #[test]
    fn test_call_without_args() {
        let output = Command::new("./target/release/tio")
            .output()
            .expect("failed to execute binary");
        assert_eq!(String::from_utf8_lossy(&output.stderr), BASE_HELP_INFO)
    }

    #[test]
    fn test_call_with_bad_args() {
        let output = Command::new("./target/release/tio")
            .arg("bad-command")
            .output()
            .expect("failed to execute binary");
        assert_eq!(String::from_utf8_lossy(&output.stderr), INVALID_COMMAND)
    }

    #[test]
    fn test_call_broadcast_without_args() {
        let output = Command::new("./target/release/tio")
            .arg("broadcast")
            .output()
            .expect("failed to broadcast message");
        assert!(String::from_utf8_lossy(&output.stdout).contains(BASE_BROADCAST_OUTPUT))
    }

    #[test]
    fn test_call_broadcast_with_args() {
        let output = Command::new("./target/release/tio")
            .args([
                "broadcast",
                "THIS IS FOR AN INTEGRATION TEST",
                "TIO INTEGRATION TEST DATA INDEX",
            ])
            .output()
            .expect("failed to broadcast message");
        assert!(String::from_utf8_lossy(&output.stdout).contains(OTHER_BROADCAST_OUTPUT))
    }

    #[test]
    fn test_call_search_without_args() {
        let output = Command::new("./target/release/tio")
            .arg("search")
            .output()
            .expect("failed to search for message");
        assert_eq!(String::from_utf8_lossy(&output.stderr), INVALID_SEARCH_ARGS)
    }

    #[test]
    fn test_call_search_with_args() {
        let output = Command::new("./target/release/tio")
            .args([
                "search",
                "977dbb8bc623d320e394987d967f5f790e102f09b87e49032e9c7704072488f5",
            ])
            .output()
            .expect("failed to search for message");
        assert!(String::from_utf8_lossy(&output.stdout).contains(BASE_SEARCH_OUTPUT))
    }
}
