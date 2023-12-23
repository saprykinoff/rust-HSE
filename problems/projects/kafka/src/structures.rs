use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ReceivedJSON {
    pub method: Option<AvailableMethods>,
    pub topic: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum  AvailableMethods {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "subscribe")]
    Subscribe
}
// impl Display for AvailableMethods {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         if (self == AvailableMethods::Subscribe) {
//             write!(f, "sub")
//         } else {
//
//         }
//     }
// }