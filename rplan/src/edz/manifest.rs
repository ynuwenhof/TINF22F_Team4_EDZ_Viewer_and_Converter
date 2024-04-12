use crate::error;
use quick_xml::de;
use serde::{Deserialize, Deserializer};
use std::path::Path;
use tokio::fs;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    #[serde(rename = "@version")]
    _version: String,
    #[serde(deserialize_with = "unwrap_packages")]
    pub packages: Vec<Package>,
}

impl Manifest {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let s = fs::read_to_string(path).await?;
        let manifest = de::from_str(&s)?;
        Ok(manifest)
    }
}

fn unwrap_packages<'de, D>(deserializer: D) -> Result<Vec<Package>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Packages {
        package: Vec<Package>,
    }

    Ok(Packages::deserialize(deserializer)?.package)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    #[serde(rename = "@type")]
    pub kind: String,
    #[serde(rename = "@key")]
    _key: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(deserialize_with = "unwrap_items")]
    pub items: Vec<Item>,
}

fn unwrap_items<'de, D>(deserializer: D) -> Result<Vec<Item>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Items {
        item: Vec<Item>,
    }

    Ok(Items::deserialize(deserializer)?.item)
}

#[derive(Deserialize)]
#[serde(tag = "@name", rename_all = "lowercase")]
pub enum Item {
    Manufacturer(ItemData),
    Supplier(ItemData),
    PictureFile(ItemData),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemData {
    #[serde(rename = "@type")]
    _kind: String,
    #[serde(rename = "@locator")]
    pub path: String,
}
