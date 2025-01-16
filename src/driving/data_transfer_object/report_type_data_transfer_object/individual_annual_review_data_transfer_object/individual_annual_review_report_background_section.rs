use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::driving::data_transfer_object::report_type_data_transfer_object::background_section_data_transfer_objects::{AdditionalCompanyMeetingAttendeeDataTransferObject, AdditionalMeetingAttendeeDataTransferObject, MeetingLocationDataTransferObject};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndividualAnnualReviewBackgroundSectionDataTransferObject{
    pub meeting_location: MeetingLocationDataTransferObject,
    pub additional_attendees: Vec<AdditionalMeetingAttendeeDataTransferObject>,
    pub additional_company_attendees: Vec<AdditionalCompanyMeetingAttendeeDataTransferObject>,
    pub meeting_date: NaiveDate
}