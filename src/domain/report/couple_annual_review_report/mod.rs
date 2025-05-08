use std::sync::Arc;

use couple_annual_review_report_sections::CoupleAnnualReviewReportSections;
use serde::{Deserialize, Serialize};

use crate::{domain::constrained_types::{adviser::Adviser, client_id::ClientId, name_string::NameString}, driven::repository::InvestmentPortfoliosRepository, driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::{couple_annual_review_report_sections_data_transfer_object::CoupleAnnualReviewReportSectionsDataTransferObject, CoupleAnnualReviewReportDataTransferObject}};
use crate::domain::DomainError;

use super::{investment_holdings::InvestmentPortfolio, ReportError};

pub mod couple_annual_review_report_sections;
pub mod couple_annual_review_report_cover_section;
pub mod couple_annual_review_report_background_section;
pub mod couple_annual_review_report_current_circumstances_section;
pub mod couple_annual_review_recommendations_section;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReport {
    sections: couple_annual_review_report_sections::CoupleAnnualReviewReportSections
}

impl CoupleAnnualReviewReport {
    pub async fn new<R>(
        unvalidated_individual_one_first_name: String,
        unvalidated_individual_one_last_name: String,
        unvalidated_individual_two_first_name: String,
        unvalidated_individual_two_last_name: String,
        unvalidated_adviser_first_name: String,
        unvalidated_adviser_last_name: String,
        unvalidated_sections: CoupleAnnualReviewReportSectionsDataTransferObject,
        investment_repo: &R
    ) -> Result<Self, ReportError> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync {

        let individual_one_first_name = NameString::try_from(unvalidated_individual_one_first_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        let individual_one_last_name = NameString::try_from(unvalidated_individual_one_last_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        let individual_two_first_name = NameString::try_from(unvalidated_individual_two_first_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        let individual_two_last_name = NameString::try_from(unvalidated_individual_two_last_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        
        let adviser = Adviser::new(
            unvalidated_adviser_first_name,
            unvalidated_adviser_last_name
        ).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;

        let couple_annual_review_report_sections = CoupleAnnualReviewReportSections::new(
            &individual_one_first_name,
            &individual_two_first_name,
            &individual_one_last_name,
            &individual_two_last_name,
            &adviser.adviser_first_name,
            &adviser.adviser_last_name,
            unvalidated_sections,
            investment_repo
        ).await?;
        
        Ok(Self {
            sections: couple_annual_review_report_sections
        })

    }

    pub async fn from_dto<R>(
        dto: CoupleAnnualReviewReportDataTransferObject,
        investment_portfolio_repo: Arc<R>
    ) -> Result<Self, ReportError> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync {

        let couple_annual_review_report = CoupleAnnualReviewReport::new(
            dto.individual_one_first_name, 
            dto.individual_one_last_name, 
            dto.individual_two_first_name, 
            dto.individual_two_last_name, 
            dto.adviser.adviser_first_name, 
            dto.adviser.adviser_last_name, 
            dto.sections, 
            investment_portfolio_repo.as_ref()
        ).await?;

        Ok(couple_annual_review_report)

    }

}

