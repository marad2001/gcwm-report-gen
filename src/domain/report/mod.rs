use std::sync::Arc;

use investment_holdings::InvestmentPortfolio;
use report_type::ReportType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use thiserror::Error;

use crate::driven::repository::InvestmentPortfoliosRepository;
use crate::driving::data_transfer_object::report_type_data_transfer_object::ReportTypeDataTransferObject;

use super::DomainError;

pub mod create_report;
pub mod report_type;
pub mod cover_section;
pub mod contents_section;
pub mod background_section;
pub mod objectives;
pub mod current_circumstances_section;
pub mod risk_assessment;
pub mod individual_annual_review_report;
pub mod couple_annual_review_report;
pub mod couple_new_report;
pub mod recommendations_section;
pub mod product;
pub mod advice_areas;
pub mod investment_holdings;

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum ReportError {
    #[error("Missing required section: {0}")]
    MissingSection(String),
    #[error("Validation error in section '{0}': {1}")]
    SectionValidationError(String, String),
    #[error("Validation error in report type '{0}': {1}")]
    ReportTypeValidationError(String, String),
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    #[error("Unexpected report error: {0}")]
    Unexpected(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    #[serde(with = "uuid::serde::simple")]
    id: uuid::Uuid,
    report_type: report_type::ReportType
}

impl Report {
    pub async fn new<R>(
        report_data: ReportTypeDataTransferObject, 
        investment_portfolio_repo: Arc<R>
    ) -> Result<Self, ReportError> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync {

        Ok(Self {
            id: Uuid::new_v4(),
            report_type: ReportType::from_dto(report_data, investment_portfolio_repo).await?
        })

    }
}


