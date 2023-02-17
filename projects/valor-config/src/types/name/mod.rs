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

impl PackageName {

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
        let mut out = PackageName::default();
        regularize(&mut out.user, user)?;
        if out.user.starts_with('-') || out.user.ends_with('-') { Err(SyntaxError::new("package user cannot start or end with `-`"))? }
        if out.user.contains("--") { Err(SyntaxError::new("package user cannot contain `--`"))? }
        regularize(&mut out.name, name)?;
        if out.name.is_empty() { Err(SyntaxError::new("package name cannot be empty"))? }
        if out.name.starts_with(|c| c.is_numeric()) { Err(SyntaxError::new("package name cannot start with a number"))? }
        if out.name.starts_with('-') || out.name.ends_with('-') { Err(SyntaxError::new("package name cannot start or end with `-`"))? }
        if out.name.contains("--") { Err(SyntaxError::new("package name cannot contain `--`"))? }
        Ok(out)
    }
}

fn regularize(buffer: &mut String, input: &str) -> Result<(), String> {
    for char in input.chars() {
        match char {
            'A'..='Z' => buffer.push(char.to_ascii_lowercase()),
            'a'..='z' => buffer.push(char),
            '0'..='9' => buffer.push(char),
            '-' | '_' | ' ' => buffer.push('-'),
            _ => return Err(format!("invalid character `{}` in package name", char)),
        }
    }
    Ok(())
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
