use codee::string::JsonSerdeCodec;
use leptos::prelude::{Signal, WriteSignal};
use leptos_use::storage::use_local_storage;

use crate::core::data::app_settings::AppSettings;

pub fn use_app_settings() -> (Signal<AppSettings>, WriteSignal<AppSettings>) {
    let (app_settings, set_app_settings, _) =
        use_local_storage::<AppSettings, JsonSerdeCodec>("settings");

    (app_settings, set_app_settings)
}
