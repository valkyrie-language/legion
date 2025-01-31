pub use crate::{authors::LegionAuthors, version::LegionVersion};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod authors;
pub mod version;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum LegionConfig {
    /// The workspace with multiple modules
    Workspace(LegionWorkspace),
    /// The package in workspace
    Module(LegionPackage),
    /// The package standalone
    Package(LegionPackage),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LegionWorkspace {
    #[serde(default)]
    pub version: LegionVersion,
    #[serde(default, alias = "commander")]
    pub authors: LegionAuthors,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LegionPackage {
    pub name: String,
    #[serde(default)]
    pub version: LegionVersion,
    #[serde(default, alias = "commander")]
    pub authors: LegionAuthors,
    pub description: Option<String>,
}
