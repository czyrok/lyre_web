#[derive(Clone)]
pub enum SelectState {
    Default,
    Active,
    Errored(String),
}
