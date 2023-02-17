use std::hash::Hash;

use super::*;

// `name`
// `@user/name`
// `@user/name-path`
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
pub struct PackageName {
    user: String,
    name: String,
}

impl Default for PackageName {
    fn default() -> Self {
        Self { user: "".to_string(), name: "".to_string() }
    }
}

impl Display for PackageName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.user.is_empty() { f.write_str(&self.name) } else { f.write_str(&format!("@{}/{}", self.user, self.name)) }
    }
}

impl FromStr for PackageName {
    type Err = SyntaxError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut user = "";
        let mut name = "";
        if input.trim().starts_with('@') {
            match input.split_once('/') {
                Some(s) => {
                    user = s.0.trim_start_matches('@');
                    name = s.1;
                },
                None => {Err(SyntaxError::new("package name must be in the format of @user/name"))?}
            }
        }
        else {
            name = input;
        }
        let out = PackageName::default();
        for (index, part) in user.split_ascii_whitespace().enumerate() {
            if index == 0 {
                out.user = part.to_string();
            }
            else {
                Err(SyntaxError::new("package name must be in the format of @user/name"))?
            }
        }
    }
}

fn parse_package_name(input: &str) -> Result<PackageName, String> {
    if s.starts_with('@') {
        let mut split = s.splitn(2, '/');
        let user = split.next().unwrap().trim_start_matches('@');
        let name = split.next().unwrap();
        Ok(PackageName { user: user.to_string(), name: name.to_string() })
    }
    else {
        Ok(PackageName { user: "".to_string(), name: s.to_string() })
    }
}

pub struct PackageNameWriter<'a> {
    ptr: &'a mut PackageName,
}

impl<'de> Deserialize<'de> for PackageName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = Self::default();
        let writer = PackageNameWriter { ptr: &mut out };
        deserializer.deserialize_any(writer)?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let writer = PackageNameWriter { ptr: place };
        deserializer.deserialize_any(writer)?;
        Ok(())
    }
}

impl<'i, 'de> Visitor<'de> for PackageNameWriter<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expect a string or a `ValorPackageName` object.")
    }

    fn visit_str<E>(mut self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match parse_package_name(v) {
            Ok(o) => *self.ptr = o,
            Err(e) => Err(E::custom(e))?,
        }
        Ok(())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "user" => self.ptr.user = value,
                "name" => self.ptr.name = value,
                _ => {}
            }
        }
        Ok(())
    }
}
