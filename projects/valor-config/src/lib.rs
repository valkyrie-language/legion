pub use errors::{Error, Result};

pub use crate::dependency::ValorDependency;
pub use crate::package::name::ValorPackageName;
pub use crate::package::ValorConfig;

mod errors;

mod package;
mod dependency;
mod workspace;
