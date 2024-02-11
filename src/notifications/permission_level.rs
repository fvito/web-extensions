use serde_derive::Serialize;
use wasm_bindgen::JsValue;
use crate::Error;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum PermissionLevel {
    /**
     * Specifies that the user has elected to show notifications from the app or extension. This is the default at install time.
     */
    Granted,
    /**
    * Specifies that the user has elected not to show notifications from the app or extension.
    */
    Denied,
}

impl TryFrom<JsValue> for PermissionLevel {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        match value.as_string().unwrap().as_str() {
            "granted" => Ok(PermissionLevel::Granted),
            "denied" => Ok(PermissionLevel::Denied),
            _=> Err(Error::Js(value))
        }
    }
}