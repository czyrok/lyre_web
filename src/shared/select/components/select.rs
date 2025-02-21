use leptos::prelude::*;

use crate::{
    core::{component_size::ComponentSize, icon_set::IconSet},
    shared::{
        button::{
            components::secondary_button::SecondaryButton,
            types::icon_side::IconSide,
        },
        components::{checkbox::Checkbox, dropdown_menu::DropdownMenu},
        select::types::select_choices_behavior::SelectChoicesBehavior,
    },
};

#[component]
pub fn Select<TSelectChoices>(
    size: ComponentSize,
    choices: TSelectChoices,
) -> impl IntoView
where
    TSelectChoices: SelectChoicesBehavior,
{
    let converted_choices = choices.list();

    choices.attach_consistency_behavior();

    view! {
        <SecondaryButton size text="Thème" on_click="dropmenu-theme-selection" icon=IconSet::SingleDownArrow icon_side=IconSide::Right anchor_name="button-theme-selection" />

        <DropdownMenu id="dropmenu-theme-selection" position_anchor_name="button-theme-selection">
            {converted_choices.into_iter()
                .map(|choice| view! {
                    <Checkbox size=ComponentSize::SM text=choice.text value=(choice.is_checked, choice.set_is_checked) can_user_unchecked=choice.can_user_unchecked />
                })
                .collect::<Vec<_>>()}
        </DropdownMenu>
    }
}
