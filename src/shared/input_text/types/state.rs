#[derive(Clone)]
pub enum InputTextState {
    Default,
    Active,
    Errored(String),
}
