use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum LegionAuthors {
    #[default]
    Inherit,
    One(String),
    Many(Vec<String>),
}
