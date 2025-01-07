use serde::{Deserialize, Serialize};

pub mod report_type_data_transfer_object;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferObject {
    pub report_type: report_type_data_transfer_object::ReportTypeDataTransferObject
}


