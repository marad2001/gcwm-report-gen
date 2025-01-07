use super::report::report_type::ReportType;

pub trait ValidatableReport {
    fn validate_data_transfer_object_data(&self) -> Result<(), String>;
    fn into_report_type(self) -> Result<ReportType, String>;
}