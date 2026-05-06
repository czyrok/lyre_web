use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectTag {
    pub short_name: String,
    pub long_name: Option<String>,
}

impl From<&String> for ProjectTag {
    fn from(name: &String) -> ProjectTag {
        ProjectTag {
            short_name: name.into(),
            long_name: None,
        }
    }
}

impl From<String> for ProjectTag {
    fn from(name: String) -> ProjectTag {
        ProjectTag {
            short_name: name,
            long_name: None,
        }
    }
}

impl From<&str> for ProjectTag {
    fn from(name: &str) -> ProjectTag {
        ProjectTag {
            short_name: name.into(),
            long_name: None,
        }
    }
}
