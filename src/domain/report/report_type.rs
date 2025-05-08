use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::report::couple_annual_review_report::CoupleAnnualReviewReport;
use crate::domain::report::individual_annual_review_report::IndividualAnnualReviewReport;
use crate::domain::report::couple_new_report::CoupleNewReport;
use crate::driven::repository::InvestmentPortfoliosRepository;
use crate::driving::data_transfer_object::report_type_data_transfer_object::ReportTypeDataTransferObject;

use super::investment_holdings::InvestmentPortfolio;
use super::ReportError;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReportType {
    CoupleAnnualReviewReport(CoupleAnnualReviewReport),
    IndividualAnnualReviewReport(IndividualAnnualReviewReport),
    CoupleNewReport(CoupleNewReport)
}

impl ReportType {

    pub async fn from_dto<R>(
        dto: ReportTypeDataTransferObject,
        investment_repo: Arc<R>
    ) -> Result<Self, ReportError>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync 
    {
        Ok(
            match dto {
                ReportTypeDataTransferObject::CoupleAnnualReviewReportDataTransferObject(inner_dto) => {
                    let inner: CoupleAnnualReviewReport = CoupleAnnualReviewReport::from_dto(inner_dto, investment_repo).await?;
                    ReportType::CoupleAnnualReviewReport(inner)
                }
                ReportTypeDataTransferObject::IndividualAnnualReviewReportDataTransferObject(inner_dto) => {
                    let inner: IndividualAnnualReviewReport = IndividualAnnualReviewReport::from_dto(inner_dto, investment_repo).await?;
                    ReportType::IndividualAnnualReviewReport(inner)
                }
                ReportTypeDataTransferObject::CoupleNewReportDto(inner_dto) => {
                    let inner: CoupleNewReport = CoupleNewReport::from_dto(inner_dto, investment_repo).await?;
                    ReportType::CoupleNewReport(inner)
                }
            }
        )
    }

}
