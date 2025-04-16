use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectTag {
    pub name: String,
}

impl From<&String> for ProjectTag {
    fn from(name: &String) -> ProjectTag {
        ProjectTag { name: name.into() }
    }
}

impl From<String> for ProjectTag {
    fn from(name: String) -> ProjectTag {
        ProjectTag { name }
    }
}

impl From<&str> for ProjectTag {
    fn from(name: &str) -> ProjectTag {
        ProjectTag { name: name.into() }
    }
}
