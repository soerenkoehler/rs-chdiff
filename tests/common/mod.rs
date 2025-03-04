// FIXME set working directory
#[macro_export]
macro_rules! run_success {
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
macro_rules! run_failure {
    ( $( $a:expr ),* ) => {
        assert_cmd::Command::cargo_bin("rs-chdiff")?
        $(
            .arg($a)
        )*
        .assert()
        .failure()
    }
}

#[macro_export]
macro_rules! assert_stdout {
    ($n:ident,$p:expr $(,$a:expr)*) => {
        #[test]
        fn $n() -> Result<(), assert_cmd::cargo::CargoError> {
            run_success!($($a),*).stdout($p);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! assert_stderr {
    ($n:ident,$p:expr $(,$a:expr)*) => {
        #[test]
        fn $n() -> Result<(), assert_cmd::cargo::CargoError> {
            run_failure!($($a),*).stderr($p);
            Ok(())
        }
    };
}
