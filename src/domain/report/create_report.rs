use std::sync::Arc;

use serde::Serialize;
use crate::domain::constrained_types::client_id::{ClientId, IoId};
use crate::domain::report::Report;
use crate::driven::repository::InvestmentPortfoliosRepository;
use crate::driving::data_transfer_object::report_type_data_transfer_object::ReportTypeDataTransferObject;

use super::investment_holdings::InvestmentPortfolio;
use super::ReportError;


pub async fn create_report<R>(
    data_transfer_object: ReportTypeDataTransferObject, 
    investment_portfolio_repo: Arc<R>
) -> Result<Report, ReportError> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> +  Sync{

    match &data_transfer_object {
        ReportTypeDataTransferObject::CoupleAnnualReviewReportDataTransferObject(couple_annual_review_data_transfer_object) => {
            
            // TODO - check for existing reports with similar names and dates of contruction - get response to continue

            let report = Report::new(data_transfer_object, investment_portfolio_repo).await?;
            
            Ok(report)

            // TODO - persist to repository

        }
        ReportTypeDataTransferObject::IndividualAnnualReviewReportDataTransferObject(individual_annual_review_data_transfer_object) => {
            
            // TODO - check for existing reports with similar names and dates of contruction - get response to continue
            
            let report = Report::new(data_transfer_object, investment_portfolio_repo).await?;
            
            Ok(report)

            // TODO - persist to repository

        }
        ReportTypeDataTransferObject::CoupleNewReportDto(couple_new_report_dto) => {
            
            // TODO - check for existing reports with similar names and dates of contruction - get response to continue
            
            let report = Report::new(data_transfer_object, investment_portfolio_repo).await?;
            
            Ok(report)

            // TODO - persist to repository

        }
    }
    
    

    

    

}