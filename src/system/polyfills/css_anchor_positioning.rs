use leptos::{
    logging::log,
    prelude::*,
    web_sys::{js_sys, window},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(
    raw_module = "/polyfills/@oddbird/css-anchor-positioning-fn@0.6.1.js"
)]
extern "C" {
    #[wasm_bindgen(js_name = "default")]
    fn polyfill(options: &JsValue);
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PolyfillOptions {
    pub elements: Option<Vec<String>>,
    pub exclude_inline_styles: bool,
    pub use_animation_frame: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HtmlStyle {
    pub anchor_name: Option<String>,
}

impl HtmlStyle {
    pub fn is_anchor_supported(&self) -> bool {
        self.anchor_name.is_some()
    }
}

#[wasm_bindgen]
pub fn apply_polyfill() -> Result<(), JsValue> {
    let window = window().ok_or("no window")?;

    let document = window.document().ok_or("no document")?;
    let document_element =
        document.document_element().ok_or("no document_element")?;

    let style = js_sys::Reflect::get(&document_element, &"style".into())?;

    let style: HtmlStyle = serde_wasm_bindgen::from_value(style)?;

    let is_anchor_supported = style.is_anchor_supported();

    log!("Anchor support: {is_anchor_supported}");

    // Check if the feature is supported
    if !is_anchor_supported {
        let default_options = PolyfillOptions {
            elements: None,
            exclude_inline_styles: false,
            use_animation_frame: false,
        };

        let default_options =
            serde_wasm_bindgen::to_value(&default_options).unwrap();

        polyfill(&default_options);
    }

    Ok(())
}
