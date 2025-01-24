use comrak::{Arena, Options};
use std::string::FromUtf8Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectContent(pub String);

impl ProjectContent {
    pub fn parse_from_str_data(data: &str) -> Result<Self, FromUtf8Error> {
        let arena = Arena::new();

        let deserialized_markdown = comrak::parse_document(&arena, data, &Options::default());

        let mut raw_content: Vec<u8> = vec![];

        comrak::format_html(deserialized_markdown, &Options::default(), &mut raw_content).unwrap();

        let content = String::from_utf8(raw_content)?;

        Ok(Self(content))
    }
}
