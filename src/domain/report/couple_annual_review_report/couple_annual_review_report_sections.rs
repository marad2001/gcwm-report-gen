use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::name_string::NameString;
use crate::domain::report::background_section::BackgroundSection;
use crate::domain::report::cover_section::CoverSection;
use crate::domain::report::contents_section::ContentsSection;
use crate::domain::report::couple_annual_review_report::couple_annual_review_report_background_section::CoupleAnnualReviewReportBackgroundSection;
use crate::domain::report::couple_annual_review_report::couple_annual_review_report_cover_section::CoupleAnnualReviewReportCoverSection;
use crate::domain::report::contents_section::AnnualReviewReportContentsSection;
use crate::domain::report::investment_holdings::InvestmentPortfolio;
use crate::domain::report::objectives::CoupleObjectivesAnnualReview;
use crate::domain::report::ReportError;
use crate::domain::DomainError;
use crate::driven::repository::InvestmentPortfoliosRepository;
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_sections_data_transfer_object::CoupleAnnualReviewReportSectionsDataTransferObject;
use crate::domain::report::current_circumstances_section::CurrentCircumstancesSection;
use crate::domain::report::recommendations_section::RecommendationsSection;

use super::couple_annual_review_recommendations_section::CoupleAnnualReviewReportRecommendationsSection;
use super::couple_annual_review_report_current_circumstances_section::CoupleAnnualReviewReportCurrentCircumstancesSection;


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportSections {
    cover: CoverSection,
    contents: ContentsSection,
    // executive_summary: ExecutiveSummarySection
    background: BackgroundSection,
    current_circumstances: CurrentCircumstancesSection,
    recommendations: RecommendationsSection
}

impl CoupleAnnualReviewReportSections {
    pub async fn new<R>(
        validated_individual_one_first_name: &NameString,
        validated_individual_two_first_name: &NameString,
        validated_individual_one_last_name: &NameString,
        validated_individual_two_last_name: &NameString,
        validated_adviser_first_name: &NameString,
        validated_adviser_last_name: &NameString,
        unvalidated_sections: CoupleAnnualReviewReportSectionsDataTransferObject,
        repo: &R
    ) -> Result<Self, ReportError> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync {

        let couple_annual_review_report_cover_section = CoverSection::CoupleAnnualReviewReportCoverSection(
            CoupleAnnualReviewReportCoverSection::new(
                validated_individual_one_first_name,
                validated_individual_one_last_name,
                validated_individual_two_first_name,
                validated_individual_two_last_name,
                validated_adviser_first_name,
                validated_adviser_last_name
            ).map_err(|(section, error)| ReportError::SectionValidationError(section, error))?
        );

        let couple_objectives_annual_review: CoupleObjectivesAnnualReview = CoupleObjectivesAnnualReview::try_from(unvalidated_sections.current_circumstances.couple_objectives).map_err(|e| DomainError::ValidationError("Couple objectives validation error".to_string()))?;

        let current_circumstances_section = CurrentCircumstancesSection::CoupleAnnualReviewReportCurrentCircumstancesSection(
            CoupleAnnualReviewReportCurrentCircumstancesSection::new(
                validated_individual_one_first_name,
                validated_individual_two_first_name,
                unvalidated_sections.current_circumstances.last_review_report_date,
                unvalidated_sections.current_circumstances.last_meeting_date,
                unvalidated_sections.current_circumstances.is_change_in_circumstances,
                &couple_objectives_annual_review,
                unvalidated_sections.current_circumstances.couple_is_risk_tolerance_change
            ).map_err(|(section, error)| ReportError::SectionValidationError(section, error))?
        );

        let recommendations_section = RecommendationsSection::CoupleAnnualReviewReportRecommendationsSection(
            CoupleAnnualReviewReportRecommendationsSection::new(
                validated_individual_one_first_name,
                validated_individual_two_first_name,
                validated_individual_one_last_name,
                validated_individual_two_last_name,
                unvalidated_sections.recommendations,
                &couple_objectives_annual_review,
                repo
            ).await.map_err(|(section, error)| ReportError::SectionValidationError(section, error))?
        );

        Ok(Self {
            cover: couple_annual_review_report_cover_section,
            contents: ContentsSection::AnnualReviewReportContentsSection(AnnualReviewReportContentsSection::new()?),
            background: BackgroundSection::CoupleAnnualReviewReportBackgroundSection(CoupleAnnualReviewReportBackgroundSection::new(unvalidated_sections.background)?),
            current_circumstances: current_circumstances_section,
            recommendations: recommendations_section
        })

    }
}