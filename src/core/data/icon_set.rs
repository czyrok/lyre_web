use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IconSet {
    Check,
    External,
    Search,
    RightArrow,
    Eye,
    Compass,
    LinkedIn,
    SingleDownArrow,
    DoubleDownArrow,
    Home,
    About,
    Email,
    Undo,
    Calendar,
    Github,
}

#[cfg(feature = "ssr")]
impl From<IconSet> for String {
    fn from(icon: IconSet) -> String {
        //// `trim_matches` removes the quotes added by `serde_json`
        serde_json::to_string(&icon)
            .unwrap()
            .trim_matches('"')
            .into()
    }
}
