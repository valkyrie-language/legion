use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter, Write},
};

use semver::VersionReq;
use serde_derive::Serialize;
use crate::PackageName;


mod der;
mod display;

#[derive(Clone, Debug, Serialize)]
pub struct DependencyResolver {
    items: BTreeMap<String, DependencyItem>,
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
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
