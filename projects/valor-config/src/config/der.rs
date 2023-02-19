use std::{collections::BTreeMap, fmt::Formatter, str::FromStr};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{DependencyItem, DependencyResolver, ValorConfig};

impl Default for ValorConfig {
    fn default() -> Self {
        Self {
            description: "".to_string(),
            authors: vec![],
            dependencies: Default::default(),
            scripts: vec![],
            files: vec![],
            main: "".to_string(),
            bin: vec![],
            keywords: vec![],
            license: "".to_string(),
            repository: "".to_string(),
            homepage: "".to_string(),
            bugs: "".to_string(),
        }
    }
}

struct ConfigWriter<'i> {
    ptr: &'i mut ValorConfig,
}

impl<'de> Deserialize<'de> for ValorConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = Self::default();
        let writer = ConfigWriter { ptr: &mut out };
        deserializer.deserialize_any(writer)?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let writer = ConfigWriter { ptr: place };
        deserializer.deserialize_any(writer)?;
        Ok(())
    }
}

impl<'i, 'de> Visitor<'de> for ConfigWriter<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a dependency object")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "description" => self.ptr.description = map.next_value()?,
                "authors" => self.ptr.authors = map.next_value()?,
                "dependencies" => self.ptr.dependencies = map.next_value()?,
                "scripts" => self.ptr.scripts = map.next_value()?,
                "files" => self.ptr.files = map.next_value()?,
                "main" => self.ptr.main = map.next_value()?,
                "bin" => self.ptr.bin = map.next_value()?,
                "keywords" => self.ptr.keywords = map.next_value()?,
                "license" => self.ptr.license = map.next_value()?,
                "repository" => self.ptr.repository = map.next_value()?,
                "homepage" => self.ptr.homepage = map.next_value()?,
                "bugs" => self.ptr.bugs = map.next_value()?,
                _ => {}
            }
        }
        Ok(())
    }
}
