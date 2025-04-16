use leptos::prelude::*;

use crate::shared::{
    button::{
        components::secondary_button::SecondaryButton,
        types::icon_side::IconSide,
    },
    components::{
        checkbox::Checkbox,
        dropdown_menu::{DropdownMenu, Position},
        icon::IconSet,
    },
    enums::component_size::ComponentSize,
    select::types::select_choices_behavior::SelectChoicesBehavior,
};

#[component]
pub fn Select(
    size: ComponentSize,
    #[prop(optional, into)] icon: Option<IconSet>,
    #[prop(optional, into)] icon_side: Option<IconSide>,
    dropdown_menu_position: Position,
    #[prop(into)] text: String,
    #[prop(into)] identifier: String,
    choices: impl SelectChoicesBehavior,
) -> impl IntoView {
    let converted_choices = choices.list();

    choices.attach_consistency_behavior();

    let drop_menu_anchor_name = format!("drop-menu-{}", identifier);
    let button_anchor_name = format!("button-{}", identifier);

    let icon = icon.unwrap_or(IconSet::SingleDownArrow);
    let icon_side = icon_side.unwrap_or(IconSide::Right);

    view! {
        <SecondaryButton size text on_click=drop_menu_anchor_name.clone() icon icon_side anchor_name=button_anchor_name.clone() />

        <DropdownMenu position=dropdown_menu_position id=drop_menu_anchor_name position_anchor_name=button_anchor_name>
            {converted_choices.into_iter()
                .map(|choice| view! {
                    <Checkbox size=ComponentSize::SM text=choice.text value=(choice.is_checked, choice.set_is_checked) can_user_unchecked=choice.can_user_unchecked />
                })
                .collect::<Vec<_>>()}
        </DropdownMenu>
    }
}
