use couple_annual_review_report::CoupleAnnualReviewReport;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::driving::data_transfer_object::report_type_data_transfer_object::ReportTypeDataTransferObject;
use crate::driving::data_transfer_object::report_type_data_transfer_object::ReportTypeDataTransferObject::CoupleAnnualReviewReportDataTransferObject;

use super::traits::ValidatableReport;

pub mod create_report;
pub mod report_type;
pub mod cover_section;
pub mod individual_annual_review_report;
pub mod couple_annual_review_report;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Report {
    #[serde(with = "uuid::serde::simple")]
    id: uuid::Uuid,
    report_type: report_type::ReportType
}

impl Report {
    pub fn new<T: ValidatableReport>(report_data: T) -> Result<Self, String> {
        
        report_data.validate_data_transfer_object_data()?;
        Ok(Self {
            id: Uuid::new_v4(),
            report_type: report_data.into_report_type()?,
        })
        
        
        //let id = Uuid::new_v4();

        // match unvalidated_report_type {
        //     CoupleAnnualReviewReportDataTransferObject(unvalidated_couple_annual_review_report) => {

        //         let unvalidated_adviser = unvalidated_couple_annual_review_report.adviser;

        //         let couple_annual_review_report = CoupleAnnualReviewReport::new(
        //             unvalidated_couple_annual_review_report.individual_one_first_name,
        //             unvalidated_couple_annual_review_report.individual_one_last_name,
        //             unvalidated_couple_annual_review_report.individual_two_first_name,
        //             unvalidated_couple_annual_review_report.individual_two_last_name,
        //             unvalidated_adviser.adviser_first_name,
        //             unvalidated_adviser.adviser_last_name
        //         );

        //         Ok(Self {
        //             id,
        //             report_type: report_type::ReportType::CoupleAnnualReviewReport(couple_annual_review_report?)
        //         })
        //     }
        // }

    }
}


