use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::name_string::NameString;
use crate::domain::report::background_section::BackgroundSection;
use crate::domain::report::cover_section::CoverSection;
use crate::domain::report::contents_section::ContentsSection;
use crate::domain::report::couple_annual_review_report::couple_annual_review_report_background_section::CoupleAnnualReviewReportBackgroundSection;
use crate::domain::report::couple_annual_review_report::couple_annual_review_report_cover_section::CoupleAnnualReviewReportCoverSection;
use crate::domain::report::contents_section::AnnualReviewReportContentsSection;
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_sections_data_transfer_object::CoupleAnnualReviewReportSectionsDataTransferObject;
use crate::domain::report::current_circumstances_section::CurrentCircumstancesSection;

use super::couple_annual_review_report_current_circumstances_section::CoupleAnnualReviewReportCurrentCircumstancesSection;


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportSections {
    cover: CoverSection,
    contents: ContentsSection,
    // executive_summary: ExecutiveSummarySection
    background: BackgroundSection,
    current_circumstances: CurrentCircumstancesSection
}

impl CoupleAnnualReviewReportSections {
    pub fn new(
        validated_individual_one_first_name: &NameString,
        validated_individual_two_first_name: &NameString,
        validated_individual_one_last_name: &NameString,
        validated_individual_two_last_name: &NameString,
        validated_adviser_first_name: &NameString,
        validated_adviser_last_name: &NameString,
        unvalidated_sections: CoupleAnnualReviewReportSectionsDataTransferObject
    ) -> Result<Self, String> {

        let couple_annual_review_report_cover_section = CoverSection::CoupleAnnualReviewReportCoverSection(
            CoupleAnnualReviewReportCoverSection::new(
                validated_individual_one_first_name,
                validated_individual_one_last_name,
                validated_individual_two_first_name,
                validated_individual_two_last_name,
                validated_adviser_first_name,
                validated_adviser_last_name
            )?
        );

        let current_circumstances_section = CurrentCircumstancesSection::CoupleAnnualReviewReportCurrentCircumstancesSection(
            CoupleAnnualReviewReportCurrentCircumstancesSection::new(
                validated_individual_one_first_name,
                validated_individual_two_first_name,
                unvalidated_sections.current_circumstances.last_review_report_date,
                unvalidated_sections.current_circumstances.last_meeting_date,
                unvalidated_sections.current_circumstances.is_change_in_circumstances,
                unvalidated_sections.current_circumstances.couple_objectives,
                unvalidated_sections.current_circumstances.couple_is_risk_tolerance_change
            )?
        );

        Ok(Self {
            cover: couple_annual_review_report_cover_section,
            contents: ContentsSection::AnnualReviewReportContentsSection(AnnualReviewReportContentsSection::new()?),
            background: BackgroundSection::CoupleAnnualReviewReportBackgroundSection(CoupleAnnualReviewReportBackgroundSection::new(unvalidated_sections.background)?),
            current_circumstances: current_circumstances_section
        })

    }
}