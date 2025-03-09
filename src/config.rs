#[cfg(test)]
mod config_test;

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
struct Config {
    excludes: Vec<String>
}
