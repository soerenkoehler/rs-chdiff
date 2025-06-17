use glob::Pattern;

use super::PatternList;

#[test]
fn serialize_patternlist() {
    let patterns = PatternList {
        patterns: vec![Pattern::new("**/*.txt").unwrap()],
    };
    assert_eq!(serde_json::to_string(&patterns).unwrap(), r#"["**/*.txt"]"#);
}

#[test]
fn deserialize_patternlist() {
    let expected = PatternList {
        patterns: vec![Pattern::new("**/*.txt").unwrap()],
    };
    assert_eq!(
        serde_json::from_reader::<&[u8], PatternList>(r#"["**/*.txt"]"#.as_bytes()).unwrap(),
        expected
    );
}

#[test]
fn deserialize_badlist() {
    match serde_json::from_reader::<&[u8], PatternList>(r#"["#.as_bytes()) {
        Ok(_) => panic!("should report error"),
        Err(err) => {
            assert!(format!("{err}").contains("EOF while parsing a list at line 1 column 1"))
        }
    }
}

#[test]
fn deserialize_badentry() {
    match serde_json::from_reader::<&[u8], PatternList>(r#"["**err/*.txt"]"#.as_bytes()) {
        Ok(_) => panic!("should report error"),
        Err(err) => {
            assert!(format!("{err}").contains("Pattern syntax error near position 2: recursive wildcards must form a single path component at line 1 column 15"))
        }
    }
}
