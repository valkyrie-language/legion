use semver::VersionReq;
use serde_derive::Serialize;

use crate::package::name::PackageName;

mod der;

#[derive(Clone, Debug, Serialize)]
pub struct ValorDependency {
    name: PackageName,
    version: VersionReq,
    path: String,
    git: String,
    branch: String,
    tag: String,
    registry: String,
}
