use driving::data_transfer_object::{self, DataTransferObject};
use lambda_http::{ext::PayloadError, http::{Response, StatusCode}, run, service_fn, Error, IntoResponse, Request, RequestExt, RequestPayloadExt};
use serde_json::{error::Category, json};
use http::Method;
use dotenv::dotenv;
use std::env;

mod domain;
mod driven;
mod driving;
mod helpers;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // need to add in tracing    
    run(service_fn(function_handler)).await
}

pub async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {

    dotenv().ok();
    
    let method = event.method();
    let path_parameters = event.path_parameters();
    println!("Path Parameters: {:?}", path_parameters.first("proxy"));

    enum PayloadType {
        Test(DataTransferObject),
        Production(Result<Option<DataTransferObject>, PayloadError>)
    }

    let payload = if path_parameters.first("proxy") == Some("test") {
        PayloadType::Test(helpers::test_helpers::create_mock_data_transfer_object())
    } else {
        match event.payload::<driving::data_transfer_object::DataTransferObject>() {
            Ok(payload) => {
                PayloadType::Production(Ok(payload))
            }
            Err(payload_error) => {
                PayloadType::Production(Err(payload_error))
            }
        }
    };

    //println!("Payload: {:?}", payload);

    match method {
        &Method::POST => {
            // if !path_parameters.is_empty() {
                
            //     let unexpected_additional_parameters_response = helpers::response_helpers::message_response("Unexpected additional parameters included");

            //     unexpected_additional_parameters_response

            // } else {

                match payload {
                    PayloadType::Test(data_transfer_object) => {
            
                        let report = domain::report::create_report::create_report(data_transfer_object.report_type);

                        let response = Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .body(json!({
                                "payload": report
                            }).to_string())
                            .map_err(Box::new)?;

                        Ok(response)

                    }
                    PayloadType::Production(payload) => {
                        match payload {
                            Ok(data_transfer_object) => {
                                match data_transfer_object {
                                    Some(data_transfer_object) => {
                                        let report = domain::report::create_report::create_report(data_transfer_object.report_type);
        
                                        let response = Response::builder()
                                            .status(StatusCode::OK)
                                            .header("Content-Type", "application/json")
                                            .body(json!({
                                                "payload": report
                                            }).to_string())
                                            .map_err(Box::new)?;
                
                                        Ok(response)
                                    }
                                    None => {
                                        let empty_payload_received_response = helpers::response_helpers::message_response("Empty payload request received");
        
                                        empty_payload_received_response
                                    }
                                }
                            }
                            Err(error) => {

                                match error {
                                    PayloadError::Json(error) => {

                                        let json_error_clasification = match error.classify() {
                                            Category::Io => "Io".to_string(),
                                            Category::Syntax => "Syntax".to_string(),
                                            Category::Data => "Data".to_string(),
                                            Category::Eof => "Eof".to_string()
                                        };

                                        let response = Response::builder()
                                            .status(StatusCode::OK)
                                            .header("Content-Type", "application/json")
                                            .body(
                                                format!(
                                                    "Json deserializing error category: {}, at line {} and column {}.", 
                                                    json_error_clasification, 
                                                    error.line().to_string(),
                                                    error.column().to_string()
                                                )
                                            )
                                            .map_err(Box::new)?;

                                        Ok(response)
                                    }
                                    PayloadError::WwwFormUrlEncoded(error) => {
                                        
                                        let response = Response::builder()
                                            .status(StatusCode::OK)
                                            .header("Content-Type", "application/json")
                                            .body(error.to_string())
                                            .map_err(Box::new)?;
                    
                                        Ok(response)
                                    }
                                }
                                
                                
                            }
                        }
                    }
                }
            }
        // }
        _ => {
            
            let no_method_context_received_response = helpers::response_helpers::message_response("No method context received by lambda function");

            no_method_context_received_response
        }
    }
    
}


