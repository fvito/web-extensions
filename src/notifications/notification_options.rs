use serde_derive::Serialize;
use crate::notifications::{NotificationItem, NotificationButton, TemplateType};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationOptions<'a> {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<&'a [NotificationButton<'a>]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_message: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<&'a u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [NotificationItem<'a>]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<&'a u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<&'a i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_interaction: Option<&'a bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<&'a bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<&'a TemplateType>,
}

impl Default for NotificationOptions<'_>  {
    fn default() -> Self {
        NotificationOptions {
            buttons: None,
            context_message: None,
            event_time: None,
            icon_url: None,
            items: None,
            message: None,
            priority: None,
            progress: None,
            requires_interaction: None,
            silent: None,
            title: None,
            kind: None,
        }
    }
}