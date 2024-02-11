use serde_derive::Serialize;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationButton<'a> {
    pub title:  &'a str
}