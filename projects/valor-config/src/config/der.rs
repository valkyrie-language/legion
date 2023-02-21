use super::*;

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
                "workspace" => {
                    self.ptr.workspace = map.next_value()?;
                    self.ptr.workspace.root = PathBuf::new();
                }
                _ => {}
            }
        }
        Ok(())
    }
}
