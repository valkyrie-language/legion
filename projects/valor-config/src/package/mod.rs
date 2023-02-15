use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::de::{Error, MapAccess, Visitor};
use serde::Deserialize;
use serde::Deserializer;
use valkyrie_errors::ValkyrieError;

use crate::ValorDependency;

pub mod name;


pub struct ValorConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub dependencies: BTreeMap<String, ValorDependency>,
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

