use crate::driving::data_transfer_object::report_type_data_transfer_object::{background_section_data_transfer_objects::{AdditionalCompanyMeetingAttendeeDataTransferObject, AdditionalMeetingAttendeeDataTransferObject, MeetingLocationDataTransferObject}, ReportTypeDataTransferObject};

use super::report::{report_type::ReportType, Report, ReportError};

pub trait Entity {}

pub trait ClientRepoId {}

pub trait ValidatableReport {
    fn validate_data_transfer_object_data(&self) -> Result<(), ReportError>;
    fn into_report_type(self) -> Result<ReportType, ReportError>;
}

pub trait BackgroundSectionDtoTrait {
    fn get_meeting_location(&self) -> &MeetingLocationDataTransferObject;
    fn get_meeting_date(&self) -> String;
    fn get_additional_attendees(&self) -> &Vec<AdditionalMeetingAttendeeDataTransferObject>;
    fn get_additional_company_attendees(&self) -> &Vec<AdditionalCompanyMeetingAttendeeDataTransferObject>;
}