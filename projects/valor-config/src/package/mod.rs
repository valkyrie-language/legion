use std::{collections::BTreeMap, fmt::Formatter, str::FromStr};

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_derive::Serialize;

use crate::ValorDependency;

mod der;
pub mod name;

#[derive(Clone, Debug, Serialize)]
pub struct ValorConfig {
    pub description: String,
    pub authors: Vec<String>,
    dependencies: BTreeMap<String, ValorDependency>,
    pub scripts: Vec<String>,
    pub files: Vec<String>,
    pub main: String,
    pub bin: Vec<String>,
    pub keywords: Vec<String>,
    pub license: String,
    pub repository: String,
    pub homepage: String,
    pub bugs: String,
}
