use crate::{LegionError, errors::LegionErrorKind};

impl From<std::io::Error> for LegionError {
    fn from(error: std::io::Error) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}

impl From<anyhow::Error> for LegionError {
    fn from(error: anyhow::Error) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}

impl From<wat::Error> for LegionError {
    fn from(error: wat::Error) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}
