use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct GUID(pub String);

impl TryFrom<String> for GUID {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref GUID_REGEX: Regex =
                Regex::new("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
                    .unwrap();
        }

        if GUID_REGEX.is_match(&value) {
            Ok(Self(value))
        } else {
            Err("The provided value is not a valid GUID.".to_owned())
        }
    }
}
