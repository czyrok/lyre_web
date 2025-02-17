use leptos::prelude::*;

#[component]
pub fn DropdownMenu(
    is_toggled: ReadSignal<bool>,
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

    let is_hidden = Memo::new(move |_| !is_toggled.get());

    view! {
        <div
            class="tw-dropdown-menu"
            class=(["tw-hidden"], move || is_hidden.get())
        >
            {items}
        </div>
    }
}
