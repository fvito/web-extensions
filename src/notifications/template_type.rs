
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateType {
    /**
    * Contains an icon, title, message, expandedMessage, and up to two buttons.
    */
    Basic,
    /**
     * Contains an icon, title, message, expandedMessage, image, and up to two buttons.
     */
    Image,
    /**
     * Contains an icon, title, message, items, and up to two buttons. Users on Mac OS X only see the first item.
     */
    List,
    /**
     * Contains an icon, title, message, progress, and up to two buttons.
     */
    Progress
}