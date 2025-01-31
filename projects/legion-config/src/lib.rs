use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum LegionConfig {
    Workspace(LegionWorkspace),
    /// The package in workspace
    Module(LegionPackage),
    /// The standalone package
    Package(LegionPackage),
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct LegionWorkspace {
    version: Option<String>,
    authors: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct LegionPackage {
    version: Option<String>,
    authors: Option<Vec<String>>,
}

#[test]
fn test() {
    let schema = schema_for!(LegionConfig);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
