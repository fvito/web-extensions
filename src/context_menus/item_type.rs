use serde_derive::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ItemType {
    #[serde(rename = "action")]
    Normal,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "separator")]
    Separator,
}