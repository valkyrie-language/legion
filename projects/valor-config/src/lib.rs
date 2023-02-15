pub use errors::{Error, Result};

pub use crate::{
    dependency::ValorDependency,
    package::{name::PackageName, ValorConfig},
};

mod errors;

mod dependency;
mod package;
mod workspace;
