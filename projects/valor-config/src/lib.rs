pub use errors::{Error, Result};

pub use crate::{
    config::ValorConfig,
    dependency::{DependencyItem, DependencyKind, DependencyResolver},
    package::ValorPackage,
    types::name::PackageName,
    workspace::ValorWorkspace,
};

mod config;
mod dependency;
mod errors;
mod package;
mod types;
mod workspace;

#[macro_use]
mod macros;
