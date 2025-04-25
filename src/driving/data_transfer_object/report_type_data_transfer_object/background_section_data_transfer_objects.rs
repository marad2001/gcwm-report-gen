use std::fmt;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct AnnualReviewBackgroundSectionDataTransferObject{
//     pub meeting_location: MeetingLocationDataTransferObject,
//     pub additional_attendees: Option<Vec<AdditionalMeetingAttendeeDataTransferObject>>,
//     pub additional_company_attendees: Option<Vec<AdditionalCompanyMeetingAttendeeDataTransferObject>>,
//     pub meeting_date: String
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MeetingLocationDataTransferObject{
    Home(HomeMeetingLocationDataTransferObject),
    Office,
    Teams,
    OtherLocationDataTransferObject(OtherLocationDataTransferObject)
}

impl fmt::Display for MeetingLocationDataTransferObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeetingLocationDataTransferObject::Home(home) => write!(f, "{}", home.town),
            MeetingLocationDataTransferObject::Office => write!(f, "office"),
            MeetingLocationDataTransferObject::Teams => write!(f, "Microsoft Teams"),
            MeetingLocationDataTransferObject::OtherLocationDataTransferObject(other) => write!(f, "{}", other.other_location),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeMeetingLocationDataTransferObject{
    pub town: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherLocationDataTransferObject{
    pub other_location: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalMeetingAttendeeDataTransferObject {
    pub first_name: String,
    pub last_name: String,
    pub relationship_to_client: RelationshipToClientDataTransferObject
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RelationshipToClientDataTransferObject {
    Accountant,
    Solicitor,
    Other(OtherRelationshipToClientDataTransferObject)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherRelationshipToClientDataTransferObject {
    pub description_of_relationship: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalCompanyMeetingAttendeeDataTransferObject {
    pub first_name: String,
    pub last_name: String
}