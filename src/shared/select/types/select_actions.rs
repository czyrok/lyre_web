use crate::core::types::closure::OnClickCallback;

pub struct SelectActions {
    pub popover_target_id: String,
    pub reset_callback: Option<Box<dyn OnClickCallback>>,
}

impl SelectActions {
    pub fn new(
        popover_target_id: String,
        reset_callback: Option<Box<dyn OnClickCallback>>,
    ) -> Self {
        Self {
            popover_target_id,
            reset_callback,
        }
    }
}

impl From<String> for SelectActions {
    fn from(popover_target_id: String) -> Self {
        Self::new(popover_target_id, None)
    }
}

impl From<&str> for SelectActions {
    fn from(popover_target_id: &str) -> Self {
        Self::new(popover_target_id.into(), None)
    }
}
