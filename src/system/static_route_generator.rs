use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
    use leptos_axum::{AxumRouteListing, StaticRouteGenerator};
    use leptos::prelude::provide_context;

    use super::app_state::AppState;
    use super::shell::shell;

    pub fn get_static_route_generator(app_state: AppState)-> (Vec<AxumRouteListing>, StaticRouteGenerator) {
        generate_route_list_with_exclusions_and_ssg_and_context(
            {
                move || shell(app_state.options.clone())
            },
            vec![].into(),
            move || {
                provide_context(app_state.project_service.clone());
            },
        )
    }
}
}
