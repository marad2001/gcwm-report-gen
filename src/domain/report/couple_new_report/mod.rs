use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::{name_string::NameString, adviser::Adviser};
use crate::domain::report::couple_new_report::couple_new_report_sections::CoupleNewReportSections;

use crate::domain::DomainError;
use crate::driven::repository::InvestmentPortfoliosRepository;
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_new_report_dto::couple_new_report_sections_dto::CoupleNewReportSectionsDto;
use crate::driving::data_transfer_object::report_type_data_transfer_object::couple_new_report_dto::CoupleNewReportDto;

use super::investment_holdings::InvestmentPortfolio;
use super::ReportError;


pub mod couple_new_report_sections;
pub mod couple_new_report_cover_section;
pub mod couple_new_report_background_section;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleNewReport {
    sections: couple_new_report_sections::CoupleNewReportSections
}

impl CoupleNewReport {
    pub async fn new<R>(
        unvalidated_individual_one_first_name: String,
        unvalidated_individual_one_last_name: String,
        unvalidated_individual_two_first_name: String,
        unvalidated_individual_two_last_name: String,
        unvalidated_adviser_first_name: String,
        unvalidated_adviser_last_name: String,
        unvalidated_sections: CoupleNewReportSectionsDto,
        investment_portfolio_repo: &R
    ) -> Result<Self, ReportError> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> {

        let individual_one_first_name = NameString::try_from(unvalidated_individual_one_first_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        let individual_one_last_name = NameString::try_from(unvalidated_individual_one_last_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        let individual_two_first_name = NameString::try_from(unvalidated_individual_two_first_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        let individual_two_last_name = NameString::try_from(unvalidated_individual_two_last_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        
        let adviser = Adviser::new(
            unvalidated_adviser_first_name,
            unvalidated_adviser_last_name
        ).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;

        let couple_new_report_sections = CoupleNewReportSections::new(
            &individual_one_first_name,
            &individual_two_first_name,
            &individual_one_last_name,
            &individual_two_last_name,
            &adviser.adviser_first_name,
            &adviser.adviser_last_name,
            unvalidated_sections
        )?;
        
        Ok(Self {
            sections: couple_new_report_sections
        })

    }

    pub async fn from_dto<R>(
        dto: CoupleNewReportDto,
        investment_portfolio_repo: Arc<R>
    ) -> Result<Self, ReportError> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync {

        let couple_annual_review_report = CoupleNewReport::new(
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