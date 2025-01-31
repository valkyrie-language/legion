pub use crate::authors::LegionAuthors;
use crate::dependencies::DependencySystem;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub mod authors;
pub mod dependencies;

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
    private: Option<bool>,
    #[serde(default)]
    version: Option<Version>,
    #[serde(default, alias = "commanders")]
    authors: LegionAuthors,
    /// Search `legion.toml` under `members` path
    #[serde(default)]
    members: Vec<String>,
    /// Add module in `include` paths
    #[serde(default)]
    include: Vec<String>,
    /// Exclude module in `exclude` paths
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(flatten)]
    dependencies: DependencySystem,
    /// Workspace level scripts
    ///
    /// The running directory is the workspace directory
    #[serde(default)]
    scripts: BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LegionPackage {
    /// The name of the package
    name: String,
    /// The version of the package
    #[serde(default)]
    version: Option<Version>,
    /// The author of the package
    #[serde(default, alias = "commanders")]
    authors: LegionAuthors,
    /// The description of the package
    description: Option<String>,
    /// The keywords of the package
    keywords: Vec<String>,
    #[serde(flatten)]
    dependencies: DependencySystem,
    #[serde(default, alias = "friends")]
    peers: DependencySystem,
    /// Package level scripts
    ///
    /// The running directory is the package directory
    #[serde(default)]
    scripts: BTreeMap<String, String>,
    #[serde(default)]
    repository: String,
    #[serde(default)]
    homepage: String,
    #[serde(default)]
    license: String,
    #[serde(default)]
    private: Option<bool>,
    #[serde(default)]
    documentation: String,
    #[serde(default)]
    readme: String,
}

impl LegionPackage {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_version(&self, ws: Option<&LegionWorkspace>) -> Option<Version> {
        match &self.version {
            Some(s) => Some(s.clone()),
            None => match ws {
                Some(s) => match &s.version {
                    Some(s) => Some(s.clone()),
                    None => None,
                },
                None => None,
            },
        }
    }
}
