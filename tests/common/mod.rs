// FIXME set working directory
#[macro_export]
macro_rules! run_binary {
    ( $( $a:expr ),* ) => {
        assert_cmd::Command::cargo_bin("rs-chdiff")?
        $(
            .arg($a)
        )*
        .assert()
        .success()
    }
}

#[macro_export]
macro_rules! assert_stdout {
    ($n:ident,$p:expr,$($a:expr),*) => {
        #[test]
        fn $n() -> Result<(), assert_cmd::cargo::CargoError> {
            run_binary!($($a),*).stdout($p);
            Ok(())
        }
    };
}
