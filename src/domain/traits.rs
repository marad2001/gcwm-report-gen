use crate::driving::data_transfer_object::report_type_data_transfer_object::{background_section_data_transfer_objects::{AdditionalCompanyMeetingAttendeeDataTransferObject, AdditionalMeetingAttendeeDataTransferObject, MeetingLocationDataTransferObject}, ReportTypeDataTransferObject};

use super::report::{create_report::CreateError, report_type::ReportType, Report};

pub trait Entity {}

pub trait ClientRepoId {}

pub trait ValidatableReport {
    fn validate_data_transfer_object_data(&self) -> Result<(), String>;
    fn into_report_type(self) -> Result<ReportType, String>;
}

pub trait ReportCreator {
    fn create(dto: ReportTypeDataTransferObject) -> Result<Report, CreateError>;
}

pub trait BackgroundSectionDtoTrait {
    fn get_meeting_location(&self) -> &MeetingLocationDataTransferObject;
    fn get_meeting_date(&self) -> String;
    fn get_additional_attendees(&self) -> &Vec<AdditionalMeetingAttendeeDataTransferObject>;
    fn get_additional_company_attendees(&self) -> &Vec<AdditionalCompanyMeetingAttendeeDataTransferObject>;
}