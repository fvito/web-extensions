use serde_derive::{Serialize};
use crate::Error;
use crate::tabs::prelude::sys;
use crate::util::{js_from_serde, object_from_js, serde_from_js};

mod context_type;
mod item_type;
mod on_clicked;
mod on_click_data;

pub use self::{
    context_type::*, item_type::*, on_clicked::*, on_click_data::*,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProperties<'a> {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<&'a [ContextType]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url_patters: Option<&'a [&'a String]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url_patters: Option<&'a [&'a String]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<ItemType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

impl Default for CreateProperties<'_>  {
    fn default() -> Self {
        CreateProperties {
            checked: None,
            contexts: None,
            document_url_patters: None,
            enabled: None,
            id: None,
            parent_id: None,
            target_url_patters: None,
            title: None,
            kind: None,
            visible: None,
        }
    }
}