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
    pub private: Option<bool>,
    #[serde(default)]
    pub version: LegionVersion,
    #[serde(default, alias = "commander")]
    pub authors: LegionAuthors,
    /// Search `legion.toml` under `members` path
    #[serde(default)]
    pub members: Vec<String>,
    /// Add module in `include` paths
    #[serde(default)]
    pub include: Vec<String>,
    /// Exclude module in `exclude` paths
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LegionPackage {
    /// The name of the package
    name: String,
    /// The version of the package
    #[serde(default)]
    version: LegionVersion,
    /// The author of the package
    #[serde(default, alias = "commander")]
    authors: LegionAuthors,
    /// The description of the package
    description: Option<String>,
    /// The keywords of the package
    keywords: Option<Vec<String>>,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    dev_dependencies: Vec<String>,
    #[serde(default)]
    build_dependencies: Vec<String>,
    #[serde(default)]
    peer_dependencies: Vec<String>,
    #[serde(default)]
    scripts: Vec<String>,
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
}
