use leptos::ev::MouseEvent;

pub enum ButtonAction {
    Callback(Box<dyn FnMut(MouseEvent)>),
    Popover(String),
    None,
}

impl<TCallback> From<TCallback> for ButtonAction
where
    TCallback: FnMut(MouseEvent) + 'static,
{
    fn from(callback: TCallback) -> ButtonAction {
        ButtonAction::Callback(Box::new(callback))
    }
}

impl From<String> for ButtonAction {
    fn from(popover_target_id: String) -> ButtonAction {
        ButtonAction::Popover(popover_target_id)
    }
}

impl From<&str> for ButtonAction {
    fn from(popover_target_id: &str) -> ButtonAction {
        ButtonAction::Popover(popover_target_id.into())
    }
}
