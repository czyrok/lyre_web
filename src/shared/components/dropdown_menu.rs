use leptos::prelude::*;

#[derive(PartialEq)]
pub enum Position {
    Right,
    Bottom,
}

#[component]
pub fn DropdownMenu(
    position: Position,
    #[prop(into)] id: String,
    #[prop(into)] position_anchor_name: String,
    children: ChildrenFragment,
) -> impl IntoView {
    let items = children()
        .nodes
        .into_iter()
        .map(|child| {
            view! {
                <div class="tw-dropdown-menu-item">
                    {child}
                </div>
            }
        })
        .collect::<Vec<_>>();

    let is_position_right = position == Position::Right;
    let is_position_bottom = position == Position::Bottom;

    view! {
        <div
            class="tw-dropdown-menu"
            class=(["tw-dropdown-menu-right"], move || is_position_right)
            class=(["tw-dropdown-menu-bottom"], move || is_position_bottom)

            id=id
            style=format!("position-anchor: --{}", position_anchor_name)
            role="menu"
            popover
        >
            {items}
        </div>
    }
}
