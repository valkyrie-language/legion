

// `@user/name`
pub struct ValorPackageName {
    user: String,
    name: String,
}

pub struct PackageNameWriter<'a> {
    ptr: &'a mut ValorPackageName,
}


impl <'de> Deserialize<'de> for ValorPackageName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let mut out = Self::default();
        let mut writer = DependencyWriter { ptr: &mut out };
        deserializer.deserialize_any(&mut writer)?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: Deserializer<'de> {
        let mut writer = DependencyWriter { ptr: place };
        deserializer.deserialize_any(&mut writer)?;
        Ok(())
    }
}