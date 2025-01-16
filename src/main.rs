use lambda_http::{http::{Response, StatusCode}, run, service_fn, Error, IntoResponse, Request, RequestExt, RequestPayloadExt};
use serde_json::json;
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
    let payload = event.payload::<driving::data_transfer_object::DataTransferObject>()?;

    match method {
        &Method::POST => {
            if !path_parameters.is_empty() {
                
                let unexpected_additional_parameters_response = helpers::response_helpers::message_response("Unexpected additional parameters included");

                unexpected_additional_parameters_response

            } else {

                match payload {
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
        }
        _ => {
            
            let no_method_context_received_response = helpers::response_helpers::message_response("No method context received by lambda function");

            no_method_context_received_response
        }
    }
    
}


