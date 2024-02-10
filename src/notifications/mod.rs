mod template_type;
mod permission_level;
mod notification_options;
mod notification_button;
mod notification_item;
mod on_button_clicked;
mod on_clicked;
mod on_closed;
mod on_permission_level_changed;

pub use web_extensions_sys as sys;
use crate::Error;
use crate::util::{js_from_serde, object_from_js, serde_from_js};


pub use self::{
    template_type::*, permission_level::*, notification_item::*, notification_button::*, notification_options::*,
    on_closed::*, on_clicked::*, on_button_clicked::*, on_permission_level_changed::*,
};

pub fn notifications() -> sys::Notifications {
    // Currently we assume a chrome browser and Manifest V3.
    //
    // Once MV3 is supported by FireFox, we need to check if we can use the same namespace,
    // a shim or our own implementation.
    sys::chrome().notifications()
}

pub fn clear(notification_id: &str) -> Result<(), Error> {
    sys::chrome()
        .notifications()
        .clear(notification_id, None);

    Ok(())
}

pub fn create(
    notification_id: &str,
    options: NotificationOptions<'_>,
) -> Result<(), Error> {
    let js_options = js_from_serde(&options)?;

    let result = sys::chrome()
        .notifications()
        .create(notification_id, object_from_js(&js_options)?, None);

    Ok(())
}

pub fn get_all() -> Result<(), Error> {
    sys::chrome()
        .notifications()
        .get_all(None);

    Ok(())
}

pub fn get_permission_level() {}

pub fn update(
    notification_id: &str,
    options: NotificationOptions<'_>,
) -> Result<(), Error> {
    let js_options = js_from_serde(&options)?;

    let result = sys::chrome()
        .notifications()
        .update(notification_id, object_from_js(&js_options)?, None);

    serde_from_js(result)
}

