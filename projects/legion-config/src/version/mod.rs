use crate::{LegionPackage, LegionWorkspace};
use schemars::{_private::NoSerialize, JsonSchema, Schema, SchemaGenerator};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::{any::type_name, borrow::Cow};

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum LegionVersion {
    #[default]
    Inherit,
    Custom(String),
}

impl JsonSchema for LegionVersion {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("LegionVersion")
    }
    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed(concat!(module_path!(), "::", "LegionVersion"))
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut map = Map::new();
        map.insert(
            "oneOf".into(),
            Value::Array({
                let mut enum_values = Vec::new();
                // enum_values.push({ generator.subschema_for::<()>() }.to_value());
                enum_values.push(
                    {
                        let mut custom = generator.subschema_for::<String>();
                        custom.insert("default".to_string(), json!("0.0.0"));
                        custom.insert("pattern".to_string(), json!("^(?:0|[1-9]\\d*)\\.(?:0|[1-9]\\d*)\\.(?:0|[1-9]\\d*)$"));
                        custom
                    }
                    .to_value(),
                );
                enum_values
            }),
        );
        Schema::from(map)
    }
}

impl LegionPackage {
    pub fn get_version(&self, ws: Option<&LegionWorkspace>) -> Option<String> {
        match &self.version {
            LegionVersion::Inherit => match ws {
                Some(s) => match &s.version {
                    LegionVersion::Inherit => None,
                    LegionVersion::Custom(s) => Some(s.clone()),
                },
                None => None,
            },
            LegionVersion::Custom(s) => Some(s.clone()),
        }
    }
}
