#[derive(Clone)]
pub struct SelectAnchorNames {
    pub button: String,
    pub dropdown_menu: String,
}

pub fn get_anchor_names(identifier: String) -> SelectAnchorNames {
    let button_anchor_name = format!("select-button-{identifier}");
    let dropdown_menu_anchor_name = format!("select-drop-menu-{identifier}");

    SelectAnchorNames {
        button: button_anchor_name,
        dropdown_menu: dropdown_menu_anchor_name,
    }
}
