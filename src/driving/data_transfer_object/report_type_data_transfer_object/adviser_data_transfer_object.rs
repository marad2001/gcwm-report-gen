use serde::{Deserialize, Serialize};
// use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdviserDataTransferObject {
    //#[serde(with = "uuid::serde::simple")]
    //id: uuid::Uuid,
    pub adviser_first_name: String,
    pub adviser_last_name: String
}