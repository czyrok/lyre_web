use serde::{Deserialize, Serialize};

const DEFAULT_PAGINATION: usize = 1;

fn get_default_pagination() -> usize {
    DEFAULT_PAGINATION
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CursorPagination {
    #[serde(default = "get_default_pagination")]
    pub limit: usize,
    pub cursor_after: Option<String>,
}
