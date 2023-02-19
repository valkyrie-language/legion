use std::{fmt::Formatter, str::FromStr};

use semver::VersionReq;
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{bind_writer, DependencyItem, DependencyResolver, PackageName};

impl Default for DependencyResolver {
    fn default() -> Self {
        Self { items: Default::default() }
    }
}

impl Default for DependencyItem {
    fn default() -> Self {
        Self {
            name: PackageName::default(),
            version: VersionReq::default(),
            path: "".to_string(),
            git: "".to_string(),
            branch: "".to_string(),
            tag: "".to_string(),
            registry: "".to_string(),
        }
    }
}

bind_writer!(DependencyResolverWriter, DependencyResolver);

impl<'i, 'de> Visitor<'de> for DependencyResolverWriter<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a dependency resolver")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "dependencies" => {
                    self.ptr.items = map.next_value()?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

bind_writer!(DependencyWriter, DependencyItem);

impl<'i, 'de> Visitor<'de> for DependencyWriter<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a dependency")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match VersionReq::from_str(v) {
            Ok(version) => {
                self.ptr.version = version;
            }
            Err(s) => Err(E::custom(s.to_string()))?,
        }
        Ok(())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => self.ptr.name = map.next_value()?,
                "version" => {
                    self.ptr.version = map.next_value()?;
                }
                "path" => {
                    self.ptr.path = map.next_value()?;
                }
                "git" => {
                    self.ptr.git = map.next_value()?;
                }
                "branch" => {
                    self.ptr.branch = map.next_value()?;
                }
                "tag" => {
                    self.ptr.tag = map.next_value()?;
                }
                "registry" => {
                    self.ptr.registry = map.next_value()?;
                }
                _ => {
                    return Err(A::Error::custom(format!("Unknown key: {}", key)));
                }
            }
        }
        Ok(())
    }
}
