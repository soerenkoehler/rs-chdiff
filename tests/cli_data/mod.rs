pub const HELP_TEXT: &str =
r#"Create, verify and compare hash sums on whole directory trees.


Usage: rs-chdiff [COMMAND]

Commands:
  create  [aliases: c]
  verify  [aliases: v]
  backup  [aliases: b]
  help    Print this message or the help of the given subcommand(s)

Options:
      --version  Print version
  -h, --help     Print help
"#;

pub const HELP_TEXT_CREATE: &str =
r#"Usage: rs-chdiff create [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -h, --help  Print help
"#;

pub const HELP_TEXT_VERIFY: &str =
r#"Usage: rs-chdiff verify [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -h, --help  Print help
"#;

pub const HELP_TEXT_BACKUP: &str =
r#"Usage: rs-chdiff backup [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -h, --help  Print help
"#;

pub const ERROR_MISSING_CMD: &str =
r#"error: Command required

Usage: rs-chdiff [COMMAND]

For more information, try '--help'.
"#;

pub const ERROR_EMPTY_CMD: &str =
r#"error: unrecognized subcommand ''

Usage: rs-chdiff [COMMAND]

For more information, try '--help'.
"#;

pub const ERROR_WRONG_CMD: &str =
r#"error: unrecognized subcommand 'unknown'

Usage: rs-chdiff [COMMAND]

For more information, try '--help'.
"#;
