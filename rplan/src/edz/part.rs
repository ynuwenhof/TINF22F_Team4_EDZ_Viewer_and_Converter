use crate::edz::attributes;
use crate::error;
use bson::oid::ObjectId;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::BufReader;

#[derive(Default, Serialize, Deserialize)]
pub struct PartContainer {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub attributes: HashMap<String, String>,
    pub part: Part,
}

impl PartContainer {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let file = File::open(path).await?;
        let reader = BufReader::new(file);

        let mut reader = Reader::from_reader(reader);
        let decoder = reader.decoder();

        let mut container = PartContainer::default();

        let mut buf = Vec::new();
        let mut level_1_buf = Vec::new();
        let mut level_2_buf = Vec::new();
        let mut level_3_buf = Vec::new();

        loop {
            match reader.read_event_into_async(&mut buf).await? {
                Event::Start(element) => {
                    container.attributes = attributes(&decoder, element.attributes())?;

                    loop {
                        level_1_buf.clear();
                        match reader.read_event_into_async(&mut level_1_buf).await? {
                            Event::Start(element) => {
                                container.part.attributes =
                                    attributes(&decoder, element.attributes())?;

                                loop {
                                    level_2_buf.clear();
                                    match reader.read_event_into_async(&mut level_2_buf).await? {
                                        Event::Start(element) => {
                                            container.part.variant.attributes =
                                                attributes(&decoder, element.attributes())?;

                                            loop {
                                                level_3_buf.clear();
                                                match reader
                                                    .read_event_into_async(&mut level_3_buf)
                                                    .await?
                                                {
                                                    Event::Empty(element) => {
                                                        container
                                                            .part
                                                            .variant
                                                            .function_templates
                                                            .push(FunctionTemplate(attributes(
                                                                &decoder,
                                                                element.attributes(),
                                                            )?));
                                                    }
                                                    Event::End(..) => break,
                                                    _ => {}
                                                }
                                            }
                                        }
                                        Event::Empty(element) => {
                                            container.part.free_properties.push(FreeProperty(
                                                attributes(&decoder, element.attributes())?,
                                            ));
                                        }
                                        Event::End(..) => break,
                                        _ => {}
                                    }
                                }
                            }
                            Event::End(..) => break,
                            _ => {}
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }

            buf.clear();
        }

        Ok(container)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Part {
    pub attributes: HashMap<String, String>,
    pub free_properties: Vec<FreeProperty>,
    pub variant: Variant,
}

#[derive(Default, Serialize, Deserialize)]
pub struct FreeProperty(pub HashMap<String, String>);

#[derive(Default, Serialize, Deserialize)]
pub struct Variant {
    pub attributes: HashMap<String, String>,
    pub function_templates: Vec<FunctionTemplate>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct FunctionTemplate(pub HashMap<String, String>);
