use serde_derive::Serialize;

use crate::package::name::ValorPackageName;

mod der;

#[derive(Clone, Debug, Serialize)]
pub struct ValorDependency {
    name: ValorPackageName,
    version: String,
    path: String,
    git: String,
    branch: String,
    tag: String,
    registry: String,
}
