pub use errors::{Error, Result};

pub use crate::{
    dependency::{DependencyItem, DependencyResolver},
    package::{name::PackageName, ValorConfig},
};

mod errors;

mod types;
mod dependency;
mod package;
mod workspace;
