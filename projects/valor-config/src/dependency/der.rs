use std::fmt::Formatter;

use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

use crate::{ValorDependency, ValorPackageName};

impl Default for ValorDependency {
    fn default() -> Self {
        Self {
            name: ValorPackageName::default(),
            version: "".to_string(),
            path: "".to_string(),
            git: "".to_string(),
            branch: "".to_string(),
            tag: "".to_string(),
            registry: "".to_string(),
        }
    }
}

struct DependencyWriter<'i> {
    ptr: &'i mut ValorDependency,
}

impl<'de> Deserialize<'de> for ValorDependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let mut out = Self::default();
        let writer = DependencyWriter { ptr: &mut out };
        deserializer.deserialize_any(writer)?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: Deserializer<'de> {
        let writer = DependencyWriter { ptr: place };
        deserializer.deserialize_any(writer)?;
        Ok(())
    }
}

impl<'i, 'de> Visitor<'de> for DependencyWriter<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}