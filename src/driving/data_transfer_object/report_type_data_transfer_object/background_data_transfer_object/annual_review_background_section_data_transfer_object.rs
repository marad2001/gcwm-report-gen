use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReviewBackgroundSectionDataTransferObject{
    meeting_location: MeetingLocationDataTransferObject,
    addtional_attendees: Vec<AdditionalMeetingAttendeeDataTransferObject>,
    additional_company_attendees: Vec<AddtionalCompanyMeetingAttendeeDataTransferObject>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MeetingLocationDataTransferObject{
    Home,
    Office,
    OtherLocationDataTrasnferObject(OtherLocationDataTransferObject)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherLocationDataTransferObject{
    other_location: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalMeetingAttendeeDataTransferObject {
    first_name: String,
    last_name: String,
    relationship_to_client: RelationshipToClientDataTransferObject
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
    description_of_relationship: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddtionalCompanyMeetingAttendeeDataTransferObject {
    first_name: String,
    last_name: String
}