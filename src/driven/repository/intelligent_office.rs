use std::collections::HashMap;
use reqwest::{self, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}, Error as ReqwestError, Client, Response};
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;

use crate::domain::constrained_types::client_id::{ClientId, IoId};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IoAddressApiResponse {
    pub href: String,
    #[serde(rename = "first_href")]
    pub first_href: String,
    pub items: Vec<AddressItem>,
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressItem {
    pub id: u32,
    pub href: String,
    #[serde(rename = "type")]
    pub address_type: String,  // Fixed reserved keyword issue
    pub resident_to: Option<String>,
    pub status: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    pub address: AddressDetails,
    #[serde(rename = "isRegisteredOnElectoralRoll")]
    pub is_registered_on_electoral_roll: Option<bool>,
    #[serde(rename = "isPotentialMortgage")]
    pub is_potential_mortgage: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressDetails {
    pub line_1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_4: Option<String>,
    pub locality: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub country: CountryDetails,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryDetails {
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IoAuthResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
    pub scope: String
}






pub async fn io_api_request(access_token: String, http_method: &str, params: Vec<&str>, body: Option<String>) -> Result<Response, ReqwestError> {

    let client = reqwest::Client::new();
    let api_key = env::var("IO_API_KEY").expect("IO_API_KEY is not set");
    let bearer_header_string = format!("Bearer {}", access_token);

    let mut url = "https://api.intelliflo.com/v2".to_string();
    for param in params {
        url = url + "/" + param
    };
    match http_method {
        "GET" => {
            client.get(url)
                .header(AUTHORIZATION, bearer_header_string)
                .header("x-api-key", api_key)
                .header(ACCEPT, "application/json")
                .send()
                .await
        }
        "POST" => {
            match body {
                Some(body) => {
                    client.post(url)
                        .header(AUTHORIZATION, bearer_header_string)
                        .header("x-api-key", api_key)
                        .header(ACCEPT, "application/json")
                        .body(body)
                        .send()
                        .await
                }
                None => {
                    client.post(url)
                        .header(AUTHORIZATION, bearer_header_string)
                        .header("x-api-key", api_key)
                        .header(ACCEPT, "application/json")
                        .send()
                        .await
                }
            }
            
        }
        _ => panic!("Error")
    }
}

pub async fn io_authenticated_client() -> Result<IoAuthResponse, ReqwestError> {

    let api_id = env::var("IO_API_ID").expect("IO_API_ID is not set");
    let api_secret = env::var("IO_API_SECRET").expect("IO_API_SECRET is not set");
    let tenant_id = env::var("IO_TENANT_ID").expect("IO_TENANT_ID is not set");
    let api_id_and_api_secret = format!("{}:{}", api_id, api_secret);
    let basic: String = general_purpose::STANDARD.encode(api_id_and_api_secret); 
    let client = reqwest::Client::new();

    let params = HashMap::from([
        ("grant_type".to_string(), "tenant_client_credentials".to_string()),
        ("scope".to_string(), "client_data client_financial_data".to_string()),
        ("tenant_id".to_string(), tenant_id.to_string())
    ]);
    let response = client.post("https://identity.gb.intelliflo.net/core/connect/token")
        .header(AUTHORIZATION, format!("Basic {}", basic))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    let deserialized_auth_response: IoAuthResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(deserialized_auth_response)

}

