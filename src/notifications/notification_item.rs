use serde_derive::Serialize;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationItem<'a> {
    /**
    *  Additional details about this item.
    */
    pub message:  &'a str,
    /**
     *  Title of one item of a list notification.
     */
    pub title:  &'a str
}