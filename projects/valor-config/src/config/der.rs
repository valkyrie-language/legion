use super::*;

impl Default for ValorConfig {
    fn default() -> Self {
        Self {
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

bind_writer!(ConfigWriter, ValorConfig);

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
                "dependencies" => self.ptr.dependencies.visit_map(&mut map, DependencyKind::Normal)?,
                "dev-dependencies" => self.ptr.dependencies.visit_map(&mut map, DependencyKind::Development)?,
                "peerDependencies" => self.ptr.dependencies.visit_map(&mut map, DependencyKind::Normal)?,
                "build-dependencies" => self.ptr.dependencies.visit_map(&mut map, DependencyKind::Build)?,
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
