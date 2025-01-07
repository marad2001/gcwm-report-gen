use serde::{Deserialize, Serialize};

use crate::driving::data_transfer_object::report_type_data_transfer_object::background_data_transfer_object::BackgroundSectionDataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct CoupleAnnualReviewReportSectionsDataTransferObject {
    background: BackgroundSectionDataTransferObject
}