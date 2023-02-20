use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_derive::Serialize;

use crate::{
    bind_writer,
    dependency::{DependencyKind, DependencyResolver},
};

mod der;

#[derive(Clone, Debug, Serialize)]
pub struct ValorConfig {
    dependencies: DependencyResolver,
    scripts: Vec<String>,
    pub files: Vec<String>,
    pub main: String,
    pub bin: Vec<String>,
    pub keywords: Vec<String>,
    pub license: String,
    pub repository: String,
    pub homepage: String,
    pub bugs: String,
}
