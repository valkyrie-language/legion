use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_derive::Serialize;
use valkyrie_errors::SyntaxError;

pub mod name;