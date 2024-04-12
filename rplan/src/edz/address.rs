use crate::error;
use quick_xml::de;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

#[derive(Deserialize)]
pub struct Address {
    #[serde(flatten)]
    attributes: HashMap<String, String>,
    #[serde(rename = "$text")]
    _text: Option<String>,
}

impl Address {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let s = fs::read_to_string(path).await?;
        let container: AnyName = de::from_str(&s)?;
        Ok(container.address)
    }

    pub fn attributes(&self) -> HashMap<String, String> {
        self.attributes
            .iter()
            .filter_map(|(key, value)| {
                let key = key
                    .starts_with("@P_PART_ADDRESS_")
                    .then(|| key.trim_start_matches("@P_PART_ADDRESS_").to_lowercase())?;

                Some((key, value.to_string()))
            })
            .collect()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnyName {
    #[serde(rename = "@count")]
    _count: String,
    #[serde(rename = "@length-unit")]
    _length_unit: String,
    #[serde(rename = "@weight-unit")]
    _weight_unit: Option<String>,
    #[serde(rename = "@type")]
    _kind: String,
    #[serde(rename = "@build")]
    _build: String,
    #[serde(rename = "@database")]
    _database: String,
    address: Address,
}
