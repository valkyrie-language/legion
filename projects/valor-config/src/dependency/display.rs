use super::*;

impl Display for DependencyItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name.to_string())
    }
}


