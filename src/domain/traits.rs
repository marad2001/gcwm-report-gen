use crate::driving::data_transfer_object::report_type_data_transfer_object::background_section_data_transfer_objects::{AdditionalCompanyMeetingAttendeeDataTransferObject, AdditionalMeetingAttendeeDataTransferObject, MeetingLocationDataTransferObject};

pub trait Entity {}



pub trait BackgroundSectionDtoTrait {
    fn get_meeting_location(&self) -> &MeetingLocationDataTransferObject;
    fn get_meeting_date(&self) -> String;
    fn get_additional_attendees(&self) -> &Option<Vec<AdditionalMeetingAttendeeDataTransferObject>>;
    fn get_additional_company_attendees(&self) -> &Option<Vec<AdditionalCompanyMeetingAttendeeDataTransferObject>>;
}