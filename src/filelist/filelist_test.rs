use std::{path::PathBuf, str::FromStr};

use crate::patternlist::PatternList;

use super::FileList;

#[test]
fn relative_excludes() {
    // FIXME invalid root path is reported directly in stderr => write
    // integration test
    FileList::from_path(
        PathBuf::from_str("tests/digest_data/skip-chdiff-txt").unwrap(),
        &PatternList::new(),
        &PatternList::new(),
    );
    todo!("assertions");
}
