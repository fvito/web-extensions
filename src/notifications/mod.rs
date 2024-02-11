mod template_type;
mod permission_level;
mod notification_options;
mod notification_button;
mod notification_item;
mod on_button_clicked;
mod on_clicked;
mod on_closed;
mod on_permission_level_changed;

use std::collections::HashMap;
pub use web_extensions_sys as sys;
use crate::Error;
use crate::util::{js_from_serde, object_from_js, serde_from_js};


pub use self::{
    template_type::*, permission_level::*, notification_item::*, notification_button::*, notification_options::*,
    on_closed::*, on_clicked::*, on_button_clicked::*, on_permission_level_changed::*,
};

pub fn notifications() -> sys::Notifications {
    // Currently we assume a Chrome browser and Manifest V3.
    //
    // Once MV3 is supported by FireFox, we need to check if we can use the same namespace,
    // a shim or our own implementation.
    sys::chrome().notifications()
}

pub async fn clear(notification_id: &str) -> Result<(), Error> {
    let result = sys::chrome()
        .notifications()
        .clear(notification_id, None)
        .await;

    serde_from_js(result)
}

pub async fn create(
    notification_id: Option<&str>,
    options: NotificationOptions<'_>,
) -> Result<String, Error> {
    let js_options = js_from_serde(&options)?;

    let result = sys::chrome()
        .notifications()
        .create(notification_id, object_from_js(&js_options)?, None)
        .await;

    serde_from_js(result)
}

pub async fn get_all() -> Result<HashMap<String, bool>, Error> {
    let result = sys::chrome()
        .notifications()
        .get_all()
        .await;

    serde_from_js(result)
}

pub async fn get_permission_level() -> Result<PermissionLevel, Error> {
    let result = sys::chrome()
        .notifications()
        .get_permission_level()
        .await;

    PermissionLevel::try_from(result)
}

pub async fn update(
    notification_id: &str,
    options: NotificationOptions<'_>,
) -> Result<(), Error> {
    let js_options = js_from_serde(&options)?;

    let result = sys::chrome()
        .notifications()
        .update(notification_id, object_from_js(&js_options)?, None)
        .await;

    serde_from_js(result)
}

