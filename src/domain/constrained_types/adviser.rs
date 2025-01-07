use serde::{Deserialize, Serialize};

use super::name_string::NameString;
// use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Adviser {
    //#[serde(with = "uuid::serde::simple")]
    //id: uuid::Uuid,
    pub adviser_first_name: NameString,
    pub adviser_last_name: NameString
}

impl Adviser {
    pub fn new(
        //id: Uuid,
        unvalidated_adviser_first_name: String,
        unvalidated_adviser_last_name: String
    ) -> Result<Self, String> {

        // TODO function to check adviser exists in database

        let adviser_first_name = NameString::try_from(unvalidated_adviser_first_name)?;
        let adviser_last_name = NameString::try_from(unvalidated_adviser_last_name)?;

        Ok(Self { adviser_first_name, adviser_last_name })

    }
}