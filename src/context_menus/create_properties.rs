use serde_derive::Serialize;
use crate::context_menus::{ContextType, ItemType};

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