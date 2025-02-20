use leptos::prelude::*;

#[component]
pub fn DropdownMenu(
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

    view! {
        <div
            class="tw-dropdown-menu tw-dropdown-menu-right"

            id=id
            style=format!("position-anchor: --{}", position_anchor_name)
            role="menu"
            popover
        >
            {items}
        </div>
    }
}
