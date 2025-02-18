use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::traits::BackgroundSectionDtoTrait;

use crate::driving::data_transfer_object::report_type_data_transfer_object::background_section_data_transfer_objects::{AdditionalCompanyMeetingAttendeeDataTransferObject, AdditionalMeetingAttendeeDataTransferObject, MeetingLocationDataTransferObject};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewBackgroundSectionDataTransferObject{
    pub meeting_location: MeetingLocationDataTransferObject,
    pub additional_attendees: Option<Vec<AdditionalMeetingAttendeeDataTransferObject>>,
    pub additional_company_attendees: Option<Vec<AdditionalCompanyMeetingAttendeeDataTransferObject>>,
    pub meeting_date: String
}

impl BackgroundSectionDtoTrait for CoupleAnnualReviewBackgroundSectionDataTransferObject {
    fn get_meeting_location(&self) -> &MeetingLocationDataTransferObject {
        &self.meeting_location
    }

    fn get_meeting_date(&self) -> String {
        self.meeting_date.clone()
    }

    fn get_additional_attendees(&self) -> &Option<Vec<AdditionalMeetingAttendeeDataTransferObject>>{
        &self.additional_attendees
    }

    fn get_additional_company_attendees(&self) -> &Option<Vec<AdditionalCompanyMeetingAttendeeDataTransferObject>> {
        &self.additional_company_attendees
    }
}