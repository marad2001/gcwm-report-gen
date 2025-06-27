use domain::report::investment_holdings::InvestmentPortfolio;
use driven::repository::{dynamo_db::{self, InvestmentPortfolioDynamoDbRepo}, InvestmentPortfoliosRepository};
use driving::data_transfer_object::{self, DataTransferObject};
use lambda_http::{ext::PayloadError, http::{Response, StatusCode}, run, service_fn, Error, IntoResponse, Request, RequestExt, RequestPayloadExt};
use tracing::{info, warn, error, instrument};
use tracing_subscriber::{fmt, EnvFilter};
use serde_json::{error::Category, json};
use http::Method;
use dotenv::dotenv;
use std::sync::Arc;

mod domain;
mod driven;
mod driving;
mod helpers;


#[tokio::main]
async fn main() -> Result<(), Error> {

    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT)
        .init();

    let dynamo_db_repo = InvestmentPortfolioDynamoDbRepo::new().await;
    let dynamo_db_repo = Arc::new(dynamo_db_repo);

    run(service_fn( move |request: Request| {
        let dynamo_db_repo = dynamo_db_repo.clone();
        async move { function_handler(request, dynamo_db_repo).await }
    }))
    .await
}

#[instrument(skip(event))]
pub async fn function_handler<R>(
    event: Request, 
    investment_portfolio_repo: Arc<R>
) -> Result<impl IntoResponse, Error> 
where 
    R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Send + Sync + 'static + std::fmt::Debug,
{

    info!(method = %event.method(), path = %event.uri(), "received request");

    let method = event.method();
    let path_parameters = event.path_parameters();

    enum PayloadType {
        Test(DataTransferObject),
        Production(Result<Option<DataTransferObject>, PayloadError>)
    }

    // let payload = if path_parameters.first("proxy") == Some("test") {
    //     PayloadType::Test(helpers::test_helpers::create_mock_data_transfer_object())
    // } else {
    //     match event.payload::<driving::data_transfer_object::DataTransferObject>() {
    //         Ok(payload) => {
    //             info!(payload = ?payload, "deserialized request payload");
    //             PayloadType::Production(Ok(payload))
    //         }
    //         Err(payload_error) => {
    //             error!(error = ?payload_error, "failed to deserialize request");
    //             PayloadType::Production(Err(payload_error))
    //         }
    //     }
    // };

    let payload = match event.payload::<driving::data_transfer_object::DataTransferObject>() {
        Ok(payload) => {
            info!(payload = ?payload, "deserialized request payload");
            PayloadType::Production(Ok(payload))
        }
        Err(payload_error) => {
            error!(error = ?payload_error, "failed to deserialize request");
            PayloadType::Production(Err(payload_error))
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
            
                        let report = domain::report::create_report::create_report(data_transfer_object.report_type, investment_portfolio_repo).await?;

                        //"REPLACE * WITH DOMAIN FOR SECURITY IN CORS");

                        let response = Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .header("Access-Control-Allow-Origin", "*")
                            .header("Access-Control-Allow-Headers", "*")
                            .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
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
                                        
                                        let report = domain::report::create_report::create_report(data_transfer_object.report_type, investment_portfolio_repo).await?;
        
                                        //"REPLACE * WITH DOMAIN FOR SECURITY IN CORS"

                                        let response = Response::builder()
                                            .status(StatusCode::OK)
                                            .header("Content-Type", "application/json")
                                            .header("Access-Control-Allow-Origin", "*")
                                            .header("Access-Control-Allow-Headers", "*")
                                            .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
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
                                            .header("Access-Control-Allow-Origin", "*")
                                            .header("Access-Control-Allow-Headers", "*")
                                            .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
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
                                            .header("Access-Control-Allow-Origin", "*")
                                            .header("Access-Control-Allow-Headers", "*")
                                            .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
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


