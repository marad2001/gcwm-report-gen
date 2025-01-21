use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::meeting_date::MeetingDate;
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_background_section_dto::CoupleAnnualReviewBackgroundSectionDataTransferObject;
use crate::domain::report::background_section::{MeetingLocation, AdditionalMeetingAttendee, AdditionalCompanyMeetingAttendee, RelationshipToClient};

use crate::helpers::text_helpers::create_background_text;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportBackgroundSection {
    background: String
}

impl CoupleAnnualReviewReportBackgroundSection {

    pub fn new(dto: CoupleAnnualReviewBackgroundSectionDataTransferObject) -> Result<Self, String> {
        let background = create_background_text(
            dto,
            "It was lovely to see you",
        )?;
        Ok(Self { background })
    }

    // pub fn new(unvalidated_couple_annual_review_report_background_section: CoupleAnnualReviewBackgroundSectionDataTransferObject) -> Result<Self, String> {

    //     let meeting_location = MeetingLocation::try_from(unvalidated_couple_annual_review_report_background_section.meeting_location)?;

    //     let meeting_date = MeetingDate::try_from(unvalidated_couple_annual_review_report_background_section.meeting_date.format("%d/%m/%Y").to_string())?.formatted_day_month();
        
    //     let additional_attendees: Result<Vec<_>, _> = unvalidated_couple_annual_review_report_background_section
    //         .additional_attendees
    //         .iter()
    //         .map(|additional_attendee_data_transfer_object| AdditionalMeetingAttendee::try_from(additional_attendee_data_transfer_object.clone()))
    //         .collect();

    //     let additional_attendees = additional_attendees?;
        
    //     let additional_company_attendees: Result<Vec<_>,_> = unvalidated_couple_annual_review_report_background_section
    //         .additional_company_attendees
    //         .iter()
    //         .map(|additional_company_meeting_attendee_data_transfer_object| AdditionalCompanyMeetingAttendee::try_from(additional_company_meeting_attendee_data_transfer_object.clone()))
    //         .collect();

    //     let additional_company_attendees = additional_company_attendees?;

    //     // Determine the greeting and location text
    //     let (greeting_text, location_text) = match meeting_location {
    //         MeetingLocation::Teams => (
    //             "It was lovely to speak to you both".to_string(),
    //             "our virtual Teams meeting".to_string()
    //         ),
    //         MeetingLocation::Home(home) => (
    //             "It was lovely to see you both".to_string(),
    //             format!("your home in {}", home.town)
    //         ),
    //         MeetingLocation::Office => (
    //             "It was lovely to see you both".to_string(),
    //             "the office".to_string()
    //         ),
    //         MeetingLocation::OtherLocation(other) => (
    //             "It was lovely to see you both".to_string(),
    //             format!("{}", other.other_location)
    //         ),
    //     };

    //     // Start building the base text
    //     let mut background_text = format!("{} at {}", greeting_text, location_text);

    //     // Adding multiple colleagues if present
    //     if additional_company_attendees.len() == 1 {
    //         let colleague = &additional_company_attendees[0];
    //         background_text.push_str(&format!(
    //             " with my colleague {} {}", colleague.first_name, colleague.last_name
    //         ));
    //     } else if additional_company_attendees.len() > 1 {
    //         let colleague_names: Vec<String> = additional_company_attendees
    //             .iter()
    //             .map(|attendee| format!("{} {}", attendee.first_name, attendee.last_name))
    //             .collect();
    //         background_text.push_str(&format!(" with my colleagues {}", colleague_names.join(", ")));
    //     }

    //     // Adding multiple other attendees if present
    //     if !additional_attendees.is_empty() {
    //         let attendee_details: Vec<String> = additional_attendees
    //             .iter()
    //             .map(|attendee| match &attendee.relationship_to_client {
    //                 RelationshipToClient::Accountant => format!("{} {}, your Accountant", attendee.first_name, attendee.last_name),
    //                 RelationshipToClient::Solicitor => format!("{} {}, your Solicitor", attendee.first_name, attendee.last_name),
    //                 RelationshipToClient::Other(other) => format!("{} {}, your {}", attendee.first_name, attendee.last_name, other.description_of_relationship)
    //             })
    //             .collect();
    //         background_text.push_str(&format!(" and {}", attendee_details.join(", ")));
    //     }

    //     // Adding the date and meeting purposes
    //     background_text.push_str(&format!(
    //         " on the {} for our regular annual review meeting. \
    //         In the meeting, we discussed your current financial position, objectives, and current invested capital that you own.\n\
    //         The remainder of this report will address a full review of your existing products in line with your financial objectives.",
    //         meeting_date
    //     ));

    //     Ok(Self { background: background_text })

    // }

}

