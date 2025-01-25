use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::meeting_date::MeetingDate;
use crate::domain::report::ReportError;
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_background_section_dto::CoupleAnnualReviewBackgroundSectionDataTransferObject;
use crate::domain::report::background_section::{MeetingLocation, AdditionalMeetingAttendee, AdditionalCompanyMeetingAttendee, RelationshipToClient};

use crate::helpers::text_helpers::create_background_text;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportBackgroundSection {
    background: String
}

impl CoupleAnnualReviewReportBackgroundSection {

    pub fn new(dto: CoupleAnnualReviewBackgroundSectionDataTransferObject) -> Result<Self, ReportError> {
        let background = create_background_text(
            dto,
            "It was lovely to see you",
        )?;
        Ok(Self { background })
    }

}

