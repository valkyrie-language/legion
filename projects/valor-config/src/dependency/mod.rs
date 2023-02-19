use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter, Write},
};

use semver::VersionReq;
use serde_derive::Serialize;
use crate::PackageName;
use std::{ str::FromStr};

use semver::{Version};
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};



mod der;
mod display;

#[derive(Clone, Debug, Serialize)]
pub struct DependencyResolver {
    items: BTreeMap<String, DependencyItem>,
}



#[derive(Clone, Debug, Serialize)]
pub struct DependencyItem {
    name: PackageName,
    version: VersionReq,
    path: String,
    git: String,
    branch: String,
    tag: String,
    registry: String,
}
