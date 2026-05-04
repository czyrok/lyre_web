use leptos::prelude::*;

use crate::{
    core::{data::icon_set::IconSet, types::closure::OnClickCallback},
    shared::{
        enums::component_size::ComponentSize,
        input_text::{
            components::unthemed_input_text::UnthemedInputText,
            helpers::active_state::effect_active_state,
            types::{state::InputTextState, theme::InputTextTheme},
        },
    },
};

#[component]
pub fn PrimaryInputText(
    size: ComponentSize,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] additional_style_classes: Option<String>,

    #[prop(name = "text")] (text, set_text): (
        ReadSignal<String>,
        WriteSignal<String>,
    ),
    #[prop(into, optional)] state: Option<RwSignal<InputTextState>>,
    #[prop(optional, into)] reset_callback: Option<Box<dyn OnClickCallback>>,
    #[prop(default = false)] shows_active_state_when_has_text: bool,
) -> impl IntoView {
    let state = state.unwrap_or(RwSignal::new(InputTextState::Default));

    if shows_active_state_when_has_text {
        effect_active_state(text, state)
    }

    view! {
        <UnthemedInputText theme=InputTextTheme::Primary text=(text, set_text) size placeholder icon additional_style_classes reset_callback state=state.into() />
    }
}
