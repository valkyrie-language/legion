use std::{collections::BTreeMap, fmt::Formatter, str::FromStr};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{DependencyItem, ValorConfig};

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
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            todo!()
        }
        Ok(())
    }
}
