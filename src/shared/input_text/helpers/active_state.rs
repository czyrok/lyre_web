use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;

use crate::shared::input_text::types::state::InputTextState;

pub fn effect_active_state(
    text: ReadSignal<String>,
    select_state: RwSignal<InputTextState>,
) {
    Effect::new(move || {
        match select_state.get() {
            InputTextState::Default => {
                let has_text = !text.get().is_empty();

                if has_text {
                    select_state.set(InputTextState::Active);
                }
            }
            InputTextState::Active => {
                let has_text = !text.get().is_empty();

                if !has_text {
                    select_state.set(InputTextState::Default);
                }
            }
            _ => (),
        };
    });
}
