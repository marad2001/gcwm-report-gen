use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::name_string::NameString;
use crate::domain::report::background_section::BackgroundSection;
use crate::domain::report::cover_section::CoverSection;
use crate::domain::report::contents_section::ContentsSection;
use crate::domain::report::individual_annual_review_report::individual_annual_review_report_cover_section::IndividualAnnualReviewReportCoverSection;
use crate::domain::report::contents_section::AnnualReviewReportContentsSection;
use crate::domain::report::individual_annual_review_report::individual_annual_review_report_background_section::IndividualAnnualReviewReportBackgroundSection;

use crate::domain::report::ReportError;
use crate::driving::data_transfer_object::report_type_data_transfer_object::individual_annual_review_data_transfer_object::individual_annual_review_report_sections_data_transfer_object::IndividualAnnualReviewReportSectionsDataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndividualAnnualReviewReportSections {
    cover: CoverSection,
    contents: ContentsSection,
    // executive_summary: ExecutiveSummarySection
    background: BackgroundSection
}

impl IndividualAnnualReviewReportSections {
    pub fn new(
        validated_individual_one_first_name: &NameString,
        validated_individual_one_last_name: &NameString,
        validated_adviser_first_name: &NameString,
        validated_adviser_last_name: &NameString,
        unvalidated_sections: IndividualAnnualReviewReportSectionsDataTransferObject
    ) -> Result<Self, ReportError> {

        let individual_annual_review_report_cover_section = CoverSection::IndividualAnnualReviewReportCoverSection(
            IndividualAnnualReviewReportCoverSection::new(
                validated_individual_one_first_name,
                validated_individual_one_last_name,
                validated_adviser_first_name,
                validated_adviser_last_name
            ).map_err(|(section, error)| ReportError::SectionValidationError(section, error))?
        );

        Ok(Self {
            cover: individual_annual_review_report_cover_section,
            contents: ContentsSection::AnnualReviewReportContentsSection(AnnualReviewReportContentsSection::new()?),
            background: BackgroundSection::IndividualAnnualReviewBackgroundSection(IndividualAnnualReviewReportBackgroundSection::new(unvalidated_sections.background)?)
        })

    }
}