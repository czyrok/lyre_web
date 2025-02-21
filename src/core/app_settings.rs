use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UsesDarkTheme(pub bool);

impl Default for UsesDarkTheme {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AppSettings {
    pub uses_dark_theme: UsesDarkTheme,
}
