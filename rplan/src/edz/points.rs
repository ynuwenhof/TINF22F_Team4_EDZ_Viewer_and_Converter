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
pub struct PointContainer {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub attributes: HashMap<String, String>,
    pub terminal: Terminal,
}

impl PointContainer {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let file = File::open(path).await?;
        let reader = BufReader::new(file);

        let mut reader = Reader::from_reader(reader);
        let decoder = reader.decoder();

        let mut container = PointContainer::default();

        let mut buf = Vec::new();
        let mut level_1_buf = Vec::new();
        let mut level_2_buf = Vec::new();

        loop {
            match reader.read_event_into_async(&mut buf).await? {
                Event::Start(element) => {
                    container.attributes = attributes(&decoder, element.attributes())?;

                    loop {
                        level_1_buf.clear();
                        match reader.read_event_into_async(&mut level_1_buf).await? {
                            Event::Start(element) => {
                                container.terminal.attributes =
                                    attributes(&decoder, element.attributes())?;

                                loop {
                                    level_2_buf.clear();
                                    match reader.read_event_into_async(&mut level_2_buf).await? {
                                        Event::Empty(element) => {
                                            container.terminal.terminal_positions.push(
                                                TerminalPosition(attributes(
                                                    &decoder,
                                                    element.attributes(),
                                                )?),
                                            );
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
        }

        Ok(container)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Terminal {
    pub attributes: HashMap<String, String>,
    pub terminal_positions: Vec<TerminalPosition>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct TerminalPosition(pub HashMap<String, String>);
