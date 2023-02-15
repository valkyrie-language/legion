use std::{collections::BTreeMap, fmt::Formatter, str::FromStr};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{ValorConfig, ValorDependency};

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
        formatter.write_str("expecting a dependency")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "description" => self.ptr.description = value,
                "authors" => self.ptr.authors = value.split(',').map(|s| s.to_string()).collect(),
                "dependencies" => {
                    let mut deps = BTreeMap::new();
                    for (key, value) in value.split(',').map(|s| s.split(':').collect::<Vec<&str>>()) {
                        deps.insert(key.to_string(), ValorDependency::from_str(value).unwrap());
                    }
                    self.ptr.dependencies = deps;
                }
                "scripts" => self.ptr.scripts = value.split(',').map(|s| s.to_string()).collect(),
                "files" => self.ptr.files = value.split(',').map(|s| s.to_string()).collect(),
                "main" => self.ptr.main = value,
                "bin" => self.ptr.bin = value.split(',').map(|s| s.to_string()).collect(),
                "keywords" => self.ptr.keywords = value.split(',').map(|s| s.to_string()).collect(),
                "license" => self.ptr.license = value,
                "repository" => self.ptr.repository = value,
                "homepage" => self.ptr.homepage = value,
                "bugs" => self.ptr.bugs = value,
                _ => {}
            }
        }
        Ok(())
    }
}
