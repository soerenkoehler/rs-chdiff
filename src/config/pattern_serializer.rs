use glob::Pattern;
use serde::{de::{self, Visitor}, ser::SerializeSeq, Deserializer, Serializer};
use std::fmt::{self, Formatter};

pub fn serialize<S>(patterns: &Vec<Pattern>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(patterns.len())).unwrap();
    patterns.into_iter().for_each(|p| {
        seq.serialize_element(p.as_str());
    });
    seq.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Pattern>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(PatternVisitor {})
}

struct PatternVisitor;

impl<'de> Visitor<'de> for PatternVisitor {
    type Value = Vec<Pattern>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string containing a valid glob pattern")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
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
