use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::name_string::NameString;
use crate::domain::report::background_section::BackgroundSection;
use crate::domain::report::cover_section::CoverSection;
use crate::domain::report::contents_section::{ContentsSection, NewReportContentsSection};

use crate::domain::report::couple_new_report::couple_new_report_cover_section::CoupleNewReportCoverSection;
use crate::domain::report::couple_new_report::couple_new_report_background_section::CoupleNewReportBackgroundSection;
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_new_report_dto::couple_new_report_sections_dto::CoupleNewReportSectionsDto;





#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleNewReportSections {
    cover: CoverSection,
    contents: ContentsSection,
    // executive_summary: ExecutiveSummarySection
    background: BackgroundSection,
    // current_circumstances: CurrentCircumstancesSection
}

impl CoupleNewReportSections {
    pub fn new(
        validated_individual_one_first_name: &NameString,
        validated_individual_two_first_name: &NameString,
        validated_individual_one_last_name: &NameString,
        validated_individual_two_last_name: &NameString,
        validated_adviser_first_name: &NameString,
        validated_adviser_last_name: &NameString,
        unvalidated_sections: CoupleNewReportSectionsDto
    ) -> Result<Self, String> {

        let couple_new_report_cover_section = CoverSection::CoupleNewReportCoverSection(
            CoupleNewReportCoverSection::new(
                validated_individual_one_first_name,
                validated_individual_one_last_name,
                validated_individual_two_first_name,
                validated_individual_two_last_name,
                validated_adviser_first_name,
                validated_adviser_last_name
            )?
        );

        Ok(Self {
            cover: couple_new_report_cover_section,
            contents: ContentsSection::NewReportContentsSection(NewReportContentsSection::new()?),
            background: BackgroundSection::CoupleNewReportBackgroundSection(CoupleNewReportBackgroundSection::new(unvalidated_sections.background)?)
        })

    }
}