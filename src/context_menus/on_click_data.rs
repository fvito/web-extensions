use serde_derive::Deserialize;
use crate::tabs::prelude::sys;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnClickData {
    pub checked: Option<bool>,
    pub editable: bool,
    pub frame_id: Option<u32>,
    pub frame_url: Option<String>,
    pub link_url: Option<String>,
    pub media_type: Option<String>,
    pub menu_item_id: Option<String>,
    pub page_url: Option<String>,
    pub parent_menu_item_id: Option<String>,
    pub selection_text: Option<String>,
    pub src_url: Option<String>,
    pub was_checked: Option<bool>,
}

impl From<sys::OnClickData> for OnClickData {
    fn from(data: sys::OnClickData) -> Self {
        Self {
            checked: data.checked(),
            editable: data.editable(),
            frame_id: data.frame_id(),
            frame_url: data.frame_url(),
            link_url: data.link_url(),
            media_type: data.media_type(),
            menu_item_id: data.menu_item_id(),
            page_url: data.page_url(),
            parent_menu_item_id: data.parent_menu_item_id(),
            selection_text: data.selection_text(),
            src_url: data.src_url(),
            was_checked: data.was_checked(),
        }
    }
}