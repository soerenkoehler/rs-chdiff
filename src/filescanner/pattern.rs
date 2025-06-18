use glob::Pattern;
use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, Visitor},
    ser::SerializeSeq,
};
use std::{
    fmt::{self, Formatter},
    path::PathBuf,
};

#[derive(PartialEq, Debug)]
pub struct PatternList {
    pub patterns: Vec<Pattern>,
}

impl PatternList {
    pub fn new() -> Self {
        Self { patterns: vec![] }
    }

    pub fn matches(&self, path: &PathBuf) -> bool {
        self.patterns
            .iter()
            .any(|pattern| pattern.matches_path(path.as_ref()))
    }

    pub fn push(&mut self, pattern: Pattern) -> &mut PatternList {
        self.patterns.push(pattern);
        self
    }
}

impl Serialize for PatternList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // TODO replace unwrap() with error handling
        let mut seq = serializer.serialize_seq(Some(self.patterns.len())).unwrap();
        self.patterns.iter().for_each(|p| {
            let _ = seq.serialize_element(p.as_str());
        });
        seq.end()
    }
}

impl<'de> Deserialize<'de> for PatternList {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.deserialize_seq(PatternVisitor {}) {
            Ok(patterns) => Ok(PatternList { patterns }),
            Err(err) => Err(de::Error::custom(err.to_string())),
        }
    }
}

struct PatternVisitor;

impl<'de> Visitor<'de> for PatternVisitor {
    type Value = Vec<Pattern>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a sequence of valid glob patterns")
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut result: Self::Value = vec![];
        loop {
            match seq.next_element::<String>() {
                Ok(None) => return Ok(result),
                Ok(Some(pattern)) => match Pattern::new(pattern.as_str()) {
                    Ok(pattern) => result.push(pattern),
                    Err(err) => return Err(de::Error::custom(err.to_string())),
                },
                Err(err) => return Err(err),
            }
        }
    }
}
