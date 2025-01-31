pub use crate::authors::LegionAuthors;
use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

pub mod authors;

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
    #[serde(default, alias = "commander")]
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
    #[serde(default)]
    peers: DependencySystem,
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
    #[serde(default, alias = "commander")]
    authors: LegionAuthors,
    /// The description of the package
    description: Option<String>,
    /// The keywords of the package
    keywords: Option<Vec<String>>,
    #[serde(flatten)]
    dependencies: DependencySystem,
    #[serde(default)]
    peers: DependencySystem,
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

#[derive(Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DependencySystem {
    #[serde(default)]
    dependencies: Vec<LegionDependency>,
    #[serde(default)]
    tests_dependencies: Vec<LegionDependency>,
    #[serde(default)]
    build_dependencies: Vec<LegionDependency>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LegionDependency {
    version: Version,
    git: Option<String>,
    branch: Option<String>,
    hash: Option<String>,
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
