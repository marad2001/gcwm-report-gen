use std::fmt;
use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::meeting_date::MeetingDate;
use crate::domain::constrained_types::{constrained_string_200::ConstrainedString200, name_string::NameString};
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_background_section::CoupleAnnualReviewBackgroundSectionDataTransferObject;
use crate::driving::data_transfer_object::report_type_data_transfer_object::background_section_data_transfer_objects::{MeetingLocationDataTransferObject, AdditionalCompanyMeetingAttendeeDataTransferObject, AdditionalMeetingAttendeeDataTransferObject, RelationshipToClientDataTransferObject, OtherRelationshipToClientDataTransferObject};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BackgroundSection {
    CoupleAnnualReviewReportBackgroundSection(CoupleAnnualReviewReportBackgroundSection),
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportBackgroundSection {
    background: String
}

impl CoupleAnnualReviewReportBackgroundSection {
    pub fn new(unvalidated_couple_annual_review_report_background_section: CoupleAnnualReviewBackgroundSectionDataTransferObject) -> Result<Self, String> {

        let meeting_location = MeetingLocation::try_from(unvalidated_couple_annual_review_report_background_section.meeting_location)?;

        let meeting_date = MeetingDate::try_from(unvalidated_couple_annual_review_report_background_section.meeting_date.format("%d/%m/%Y").to_string())?;
        
        let additional_attendees: Result<Vec<_>, _> = unvalidated_couple_annual_review_report_background_section
            .additional_attendees
            .iter()
            .map(|additional_attendee_data_transfer_object| AdditionalMeetingAttendee::try_from(additional_attendee_data_transfer_object.clone()))
            .collect();

        let additional_attendees = additional_attendees?;
        
        let additional_company_attendees: Result<Vec<_>,_> = unvalidated_couple_annual_review_report_background_section
            .additional_company_attendees
            .iter()
            .map(|additional_company_meeting_attendee_data_transfer_object| AdditionalCompanyMeetingAttendee::try_from(additional_company_meeting_attendee_data_transfer_object.clone()))
            .collect();

        let additional_company_attendees = additional_company_attendees?;

        // Determine the greeting and location text
        let (greeting_text, location_text) = match meeting_location {
            MeetingLocation::Teams => (
                "It was lovely to speak to you".to_string(),
                "our virtual Teams meeting".to_string()
            ),
            MeetingLocation::Home(home) => (
                "It was lovely to see you".to_string(),
                format!("your home in {}", home.town)
            ),
            MeetingLocation::Office => (
                "It was lovely to see you".to_string(),
                "the office".to_string()
            ),
            MeetingLocation::OtherLocation(other) => (
                "It was lovely to see you".to_string(),
                format!("{}", other.other_location)
            ),
        };

        // Start building the base text
        let mut background_text = format!("{} at {}", greeting_text, location_text);

        // Adding multiple colleagues if present
        if !additional_company_attendees.is_empty() {
            let colleague_names: Vec<String> = additional_company_attendees
                .iter()
                .map(|attendee| format!("{} {}", attendee.first_name, attendee.last_name))
                .collect();
            background_text.push_str(&format!(" with my colleagues {}", colleague_names.join(", ")));
        }

        // Adding multiple other attendees if present
        if !additional_attendees.is_empty() {
            let attendee_details: Vec<String> = additional_attendees
                .iter()
                .map(|attendee| match &attendee.relationship_to_client {
                    RelationshipToClient::Accountant => format!("{} {}, your Accountant", attendee.first_name, attendee.last_name),
                    RelationshipToClient::Solicitor => format!("{} {}, your Solicitor", attendee.first_name, attendee.last_name),
                    RelationshipToClient::Other(other) => format!("{} {}, your {}", attendee.first_name, attendee.last_name, other.description_of_relationship)
                })
                .collect();
            background_text.push_str(&format!(" and {}", attendee_details.join(", ")));
        }

        // Adding the date and meeting purposeS
        background_text.push_str(&format!(
            " on the {} for our regular annual review meeting. \
            In the meeting, we discussed your current financial position, objectives, and current invested capital that you own.\n\
            The remainder of this report will address a full review of your existing products in line with your financial objectives.",
            meeting_date
        ));

        Ok(Self { background: background_text })

    }

    /// Returns the background text
    pub fn get_background_text(&self) -> &String {
        &self.background
    }

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
    town: ConstrainedString200,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherLocation{
    other_location: ConstrainedString200,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalMeetingAttendee {
    first_name: NameString,
    last_name: NameString,
    relationship_to_client: RelationshipToClient
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
    description_of_relationship: ConstrainedString200
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
    first_name: NameString,
    last_name: NameString
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