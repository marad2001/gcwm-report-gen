use std::fmt;
use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::{constrained_string_200::ConstrainedString200, name_string::NameString};
use super::couple_annual_review_report::couple_annual_review_report_background_section::CoupleAnnualReviewReportBackgroundSection;
use super::individual_annual_review_report::individual_annual_review_report_background_section::IndividualAnnualReviewReportBackgroundSection;
use crate::driving::data_transfer_object::report_type_data_transfer_object::background_section_data_transfer_objects::{MeetingLocationDataTransferObject, AdditionalCompanyMeetingAttendeeDataTransferObject, AdditionalMeetingAttendeeDataTransferObject, RelationshipToClientDataTransferObject, OtherRelationshipToClientDataTransferObject};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BackgroundSection {
    CoupleAnnualReviewReportBackgroundSection(CoupleAnnualReviewReportBackgroundSection),
    IndividualAnnualReviewBackgroundSection(IndividualAnnualReviewReportBackgroundSection)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MeetingLocation{
    Home(HomeLocation),
    Office,
    Teams,
    OtherLocation(OtherLocation)
}

impl fmt::Display for MeetingLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeetingLocation::Home(home) => write!(f, "{}", home.town),
            MeetingLocation::Office => write!(f, "office"),
            MeetingLocation::Teams => write!(f, "Microsoft Teams"),
            MeetingLocation::OtherLocation(other) => write!(f, "{}", other.other_location),
        }
    }
}

impl TryFrom<MeetingLocationDataTransferObject> for MeetingLocation {
    type Error = String;

    fn try_from(value: MeetingLocationDataTransferObject) -> Result<Self, Self::Error> {
        match value {
            MeetingLocationDataTransferObject::Home(home_meeting_location_data_transfer_object) => {
                Ok(MeetingLocation::Home(
                    Ok::<HomeLocation, String>(HomeLocation{ 
                            town: ConstrainedString200::try_from(home_meeting_location_data_transfer_object.town)? 
                        }
                    )?
                ))
            },
            MeetingLocationDataTransferObject::Office => Ok(MeetingLocation::Office),
            MeetingLocationDataTransferObject::Teams => Ok(MeetingLocation::Teams),
            MeetingLocationDataTransferObject::OtherLocationDataTransferObject(other_location_data_transfer_object) => {
                Ok(MeetingLocation::OtherLocation(
                        Ok::<OtherLocation, String>(OtherLocation{
                                other_location: ConstrainedString200::try_from(other_location_data_transfer_object.other_location)?
                            }
                        )?
                    )
                )
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeLocation{
    pub town: ConstrainedString200,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherLocation{
    pub other_location: ConstrainedString200,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalMeetingAttendee {
    pub first_name: NameString,
    pub last_name: NameString,
    pub relationship_to_client: RelationshipToClient
}

impl TryFrom<AdditionalMeetingAttendeeDataTransferObject> for AdditionalMeetingAttendee {
    type Error = String;

    fn try_from(value: AdditionalMeetingAttendeeDataTransferObject) -> Result<Self, Self::Error> {
        let relationship_to_client = RelationshipToClient::try_from(value.relationship_to_client)?;
        
        Ok(Self {
            first_name: NameString::try_from(value.first_name)?,
            last_name: NameString::try_from(value.last_name)?,
            relationship_to_client
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RelationshipToClient {
    Accountant,
    Solicitor,
    Other(OtherRelationshipToClient)
}

impl TryFrom<RelationshipToClientDataTransferObject> for RelationshipToClient {
    type Error = String;

    fn try_from(value: RelationshipToClientDataTransferObject) -> Result<Self, Self::Error> {
        match value {
            RelationshipToClientDataTransferObject::Accountant => Ok(RelationshipToClient::Accountant),
            RelationshipToClientDataTransferObject::Solicitor => Ok(RelationshipToClient::Solicitor),
            RelationshipToClientDataTransferObject::Other(other_relationship_to_client_data_transfer_object) => {
                Ok(RelationshipToClient::Other(OtherRelationshipToClient::try_from(other_relationship_to_client_data_transfer_object)?))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherRelationshipToClient {
    pub description_of_relationship: ConstrainedString200
}

impl TryFrom<OtherRelationshipToClientDataTransferObject> for OtherRelationshipToClient {
    type Error = String;

    fn try_from(value: OtherRelationshipToClientDataTransferObject) -> Result<Self, Self::Error> {
        Ok(OtherRelationshipToClient {
            description_of_relationship: ConstrainedString200::try_from(value.description_of_relationship)?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalCompanyMeetingAttendee {
    pub first_name: NameString,
    pub last_name: NameString
}

impl TryFrom<AdditionalCompanyMeetingAttendeeDataTransferObject> for AdditionalCompanyMeetingAttendee {
    type Error = String;

    fn try_from(value: AdditionalCompanyMeetingAttendeeDataTransferObject) -> Result<Self, Self::Error> {
        Ok(Self{
            first_name: NameString::try_from(value.first_name)?,
            last_name: NameString::try_from(value.last_name)?
        })
    }
}