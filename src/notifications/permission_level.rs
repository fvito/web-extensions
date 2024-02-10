use serde_derive::Serialize;
use wasm_bindgen::JsValue;

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

impl From<JsValue> for PermissionLevel {
    fn from(value: JsValue) -> Self {
        let value = value.as_string().expect("Given input is not a string");
        match value.as_str() {
            "granted" => PermissionLevel::Granted,
            "denied" => PermissionLevel::Denied,
            _=> panic!("Unknown permission level")
        }
    }
}