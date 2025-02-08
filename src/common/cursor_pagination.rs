use serde::{Deserialize, Serialize};

const DEFAULT_PAGINATION_LIMIT: u32 = 1;

fn get_default_pagination_limit() -> u32 {
    DEFAULT_PAGINATION_LIMIT
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CursorPagination {
    #[serde(default = "get_default_pagination_limit")]
    pub limit: u32,
    pub cursor_after: Option<String>,
}
