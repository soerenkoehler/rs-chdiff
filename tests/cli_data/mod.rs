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
r#"Usage: rs-chdiff create [OPTIONS] [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -a, --algorithm <ALGORITHM>  [default: sha256] [possible values: sha256, sha512]
  -h, --help                   Print help
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
