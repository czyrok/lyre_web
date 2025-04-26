use leptos::{ev::MouseEvent, prelude::*};

use crate::shared::{
    button::components::primary_button::PrimaryButton,
    enums::component_size::ComponentSize,
};

#[component]
pub fn MorePagination(
    count_left: Signal<usize>,
    on_click: impl FnMut(MouseEvent) + 'static,
    #[prop(optional, into)] click_text: Option<String>,
) -> impl IntoView {
    let click_text = click_text.unwrap_or("Voir Plus".into());

    view! {
        <div class="tw-pagination">
            <PrimaryButton size=ComponentSize::MD text=click_text on_click=on_click />

            <span class="tw-pagination-text">
                {move || count_left.get()}
                <Show
                    when=move || { count_left.get() > 1 }
                    fallback=move || view! {
                        " restant."
                    }
                >
                    " restants."
                </Show>
            </span>
        </div>
    }
}
