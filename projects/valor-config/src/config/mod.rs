use std::{
    ffi::OsStr,
    fmt::{Display, Formatter},
    fs::read_to_string,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_derive::Serialize;
use valkyrie_errors::ValkyrieResult;

use crate::{bind_writer, DependencyKind, DependencyResolver, ValorPackage, ValorWorkspace};

mod der;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ValorConfig {
    workspace: ValorWorkspace,
    package: ValorPackage,
    scripts: Vec<String>,
    dependencies: DependencyResolver,
}

pub enum SupportFormat {
    Nothing,
    Toml,
    Json,
}

impl ValorConfig {
    pub fn is_workspace(&self) -> bool {
        self.workspace.root != PathBuf::from("<<MISSING>>")
    }
    pub fn is_template(&self) -> bool {
        !self.package.is_valid()
    }
    pub fn is_package(&self) -> bool {
        !self.is_workspace() && !self.is_template()
    }
}
