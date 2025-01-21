use crate::domain::traits::BackgroundSectionDtoTrait;
use crate::domain::report::background_section::{MeetingLocation, AdditionalCompanyMeetingAttendee, AdditionalMeetingAttendee};
use crate::domain::constrained_types::meeting_date::MeetingDate;
use crate::domain::report::background_section::RelationshipToClient;

pub fn create_background_text<T>(
    dto: T,
    base_greeting: &str, // Base greeting for all cases
) -> Result<String, String>
where
    T: BackgroundSectionDtoTrait,
{
    let meeting_location = MeetingLocation::try_from(dto.get_meeting_location().clone())?;
    let meeting_date = MeetingDate::try_from(dto.get_meeting_date())?.formatted_day_month();

    let additional_attendees: Result<Vec<_>, _> = dto
        .get_additional_attendees()
        .iter()
        .map(|a| AdditionalMeetingAttendee::try_from(a.clone()))
        .collect();
    let additional_company_attendees: Result<Vec<_>, _> = dto
        .get_additional_company_attendees()
        .iter()
        .map(|a| AdditionalCompanyMeetingAttendee::try_from(a.clone()))
        .collect();

    let additional_attendees = additional_attendees?;
    let additional_company_attendees = additional_company_attendees?;

    // Determine the greeting and location text
    let location_text = match meeting_location {
        MeetingLocation::Teams => "our virtual Teams meeting".to_string(),
        MeetingLocation::Home(home) => format!("your home in {}", home.town),
        MeetingLocation::Office => "the office".to_string(),
        MeetingLocation::OtherLocation(other) => format!("{}", other.other_location),
    };

    // Build greeting text dynamically
    let mut greeting_text = base_greeting.to_string();

    if !additional_company_attendees.is_empty() {
        let company_names: Vec<String> = additional_company_attendees
            .iter()
            .map(|a| format!("{} {}", a.first_name, a.last_name))
            .collect();
        greeting_text.push_str(&format!(" with my colleagues {}", company_names.join(", ")));
    }

    if !additional_attendees.is_empty() {
        let attendee_details: Vec<String> = additional_attendees
            .iter()
            .map(|a| match &a.relationship_to_client {
                RelationshipToClient::Accountant => format!("{} {}, your Accountant", a.first_name, a.last_name),
                RelationshipToClient::Solicitor => format!("{} {}, your Solicitor", a.first_name, a.last_name),
                RelationshipToClient::Other(other) => format!("{} {}, your {}", a.first_name, a.last_name, other.description_of_relationship),
            })
            .collect();
        greeting_text.push_str(&format!(" and {}", attendee_details.join(", ")));
    }

    let mut background_text = format!("{} at {}", greeting_text, location_text);

    // Add meeting date and conclusion
    background_text.push_str(&format!(
        " on the {} for our regular annual review meeting. In the meeting, we discussed your current financial position, objectives, and invested capital that you own.\nThe remainder of this report will address a full review of your existing products in line with your financial objectives.",
        meeting_date
    ));

    Ok(background_text)
}


