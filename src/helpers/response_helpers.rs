use http::{Response, StatusCode};
use lambda_http::Error;
use serde_json::json;



pub fn message_response(message: &str) -> Result<Response<String>, Error> {

    let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(json!({
                        "message": message,
                    }).to_string())
                    .map_err(Box::new)?;
    
    Ok(response)

}
