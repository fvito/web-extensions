use serde_derive::{Serialize};
use crate::Error;
use crate::util::{js_from_serde, object_from_js, serde_from_js};
pub use web_extensions_sys as sys;

mod context_type;
mod item_type;
mod on_clicked;
mod on_click_data;
mod create_properties;

pub use self::{
    context_type::*, item_type::*, on_clicked::*, on_click_data::*, create_properties::*
};

pub fn context_menus() -> sys::ContextMenus {
    // Currently we assume a chrome browser and Manifest V3.
    //
    // Once MV3 is supported by FireFox, we need to check if we can use the same namespace,
    // a shim or our own implementation.
    sys::chrome().context_menus()
}

pub fn clear_all_menus() -> Result<(), Error> {
    sys::chrome()
        .context_menus()
        .remove_all(None);
    Ok(())
}

pub fn create(props: CreateProperties<'_>) -> Result<String, Error> {
    let js_props = js_from_serde(&props)?;
    let result = sys::chrome()
        .context_menus()
        .create(object_from_js(&js_props)?, None);

    serde_from_js(result)
}

pub fn remove(menu_item_id: &String) -> Result<(), Error> {
    let id = js_from_serde(menu_item_id)?;
    sys::chrome()
        .context_menus()
        .remove(&id, None);

    Ok(())
}


pub fn update(menu_item_id: &String, update_properties: &CreateProperties<'_>) -> Result<(), Error> {
    let js_props = js_from_serde(&update_properties)?;
    let js_id = js_from_serde(&menu_item_id)?;
    sys::chrome()
        .context_menus()
        .update(&js_id, object_from_js(&js_props)?, None);

    Ok(())
}