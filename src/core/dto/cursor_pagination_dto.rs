use serde::{Deserialize, Serialize};

const DEFAULT_PAGINATION_LIMIT: u32 = 12;

fn get_default_pagination_limit() -> u32 {
    DEFAULT_PAGINATION_LIMIT
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CursorPaginationDto {
    #[serde(default = "get_default_pagination_limit")]
    pub limit: u32,
    pub cursor_after: Option<String>,
}

impl CursorPaginationDto {
    pub fn new_from_cursor(cursor_after: Option<String>) -> Self {
        Self {
            limit: get_default_pagination_limit(),
            cursor_after,
        }
    }

    pub fn new_from_limit(limit: u32) -> Self {
        Self {
            limit,
            cursor_after: None,
        }
    }
}

impl Default for CursorPaginationDto {
    fn default() -> Self {
        Self {
            limit: get_default_pagination_limit(),
            cursor_after: None,
        }
    }
}
