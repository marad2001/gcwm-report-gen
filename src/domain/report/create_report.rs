use serde::Serialize;
use crate::domain::constrained_types::client_id::{ClientId, IoId};
use crate::domain::report::Report;
use crate::driven::repository::MainContactAddress;
use crate::driving::data_transfer_object::report_type_data_transfer_object::ReportTypeDataTransferObject;


#[derive(Serialize, Debug)]
pub enum CreateError {
    InvalidData(String),
}

pub fn create_report(data_transfer_object: ReportTypeDataTransferObject) -> Result<Report, CreateError> {

    match data_transfer_object {
        ReportTypeDataTransferObject::CoupleAnnualReviewReportDataTransferObject(couple_annual_review_data_transfer_object) => {
            
            // TODO - check for existing reports with similar names and dates of contruction - get response to continue

            let report = Report::new(couple_annual_review_data_transfer_object)
            .map_err(|e| CreateError::InvalidData(e))?;
            
            Ok(report)

            // TODO - persist to repository

        }
        ReportTypeDataTransferObject::IndividualAnnualReviewReportDataTransferObject(individual_annual_review_data_transfer_object) => {
            
            // TODO - check for existing reports with similar names and dates of contruction - get response to continue
            
            let report = Report::new(individual_annual_review_data_transfer_object)
            .map_err(|e| CreateError::InvalidData(e))?;
            
            Ok(report)

            // TODO - persist to repository

        }
    }
    
    

    

    

}