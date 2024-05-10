mod address;
mod manifest;
mod part;
mod points;

pub use self::address::*;
pub use self::manifest::*;
pub use self::part::*;
pub use self::points::*;

use crate::error;
use quick_xml::events::attributes::Attributes;
use quick_xml::Decoder;
use std::collections::HashMap;

pub(super) fn attributes(
    decoder: &Decoder,
    attributes: Attributes,
) -> error::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for attribute in attributes {
        let attribute = attribute?;

        let key = decoder.decode(attribute.key.0)?;
        let value = decoder.decode(attribute.value.as_ref())?;
        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
}
