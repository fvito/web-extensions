use serde_derive::Serialize;

// https://developer.chrome.com/docs/extensions/reference/api/contextMenus#type-ContextType
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ContextType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "frame")]
    Frame,
    #[serde(rename = "selection")]
    Selection,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "editable")]
    Editable,
    #[serde(rename = "image", )]
    Image,
    #[serde(rename = "video", )]
    Video,
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "launcher")]
    Launcher,
    #[serde(rename = "browser_action")]
    BrowserAction,
    #[serde(rename = "page_action")]
    PageAction,
    #[serde(rename = "action")]
    Action,
}