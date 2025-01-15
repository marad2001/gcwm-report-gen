use async_trait::async_trait;
use intelligent_office::{IoAddressApiResponse};
use serde::{Deserialize, Serialize};

use crate::{domain::{constrained_types::client_id::ClientId, report::Report, traits::{ClientRepoId, Entity}}, main};
use crate::driven::repository::intelligent_office::io_authenticated_client;
use crate::driven::repository::intelligent_office::io_api_request;

pub mod intelligent_office;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReport {
    pub id: Option<String>,
    pub report: Report,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MainContactAddress {
    pub address_line_one: String,
    pub address_line_two: Option<String>,
    pub address_line_three: Option<String>,
    pub address_line_four: Option<String>,
    pub town_or_city: String,
    pub postcode: String
}

impl TryFrom<IoAddressApiResponse> for MainContactAddress {
    type Error = String;

    fn try_from(io_address_api_response: IoAddressApiResponse) -> Result<Self, Self::Error> {

        let mut main_contact_address = Vec::new();
        for address_item in io_address_api_response.items {
            if address_item.is_default == true {
                main_contact_address.push(Self{
                    address_line_one: address_item.address.line_1,
                    address_line_two: if address_item.address.line_2.is_some() { address_item.address.line_2 } else { None },
                    address_line_three: if address_item.address.line_3.is_some() { address_item.address.line_3 } else { None },
                    address_line_four: if address_item.address.line_4.is_some() { address_item.address.line_4 } else { None },
                    town_or_city: address_item.address.locality,
                    postcode: address_item.address.postal_code
                })
            }
        }

        if main_contact_address.len() < 1 {
            Err("More than one main address found".to_string())
        } else if main_contact_address.len() == 0 {
            Err("No main contact address found".to_string())
        } else {
            Ok(main_contact_address[0].clone())
        }
    }

}

#[async_trait]
impl QueryExternalRepository<ClientId> for MainContactAddress {

    async fn get_clients_main_contact_address(id: ClientId) -> Result<MainContactAddress, String> {
        match id {
            ClientId::IoId(io_id) => {

                let io_authenticated_client = io_authenticated_client().await;
                
                match io_authenticated_client {
                    Ok(io_auth_response) => {
                        // Constructing the API request URL dynamically using io_id
                        let io_id_string = io_id.to_string();
                        let io_api_request_contact_address = io_api_request(
                            io_auth_response.access_token, 
                            "GET", 
                            vec!["clients", &io_id_string , "addresses"], 
                            None
                        );
                        
                        // Proper error handling using ? instead of unwrap
                        let response = io_api_request_contact_address.await
                            .map_err(|_| "Failed to send request".to_string())?
                            .text().await
                            .map_err(|_| "Failed to parse response text".to_string())?;

                        println!("IO main contact address response: {}", response);
                        
                        // Deserialize response into MainContactAddress
                        let io_address_api_response: IoAddressApiResponse = serde_json::from_str(&response)
                            .map_err(|e| format!("Failed to deserialize response with error {} at line {} and column {}", e, e.line(), e.column()))?;

                        Ok(MainContactAddress::try_from(io_address_api_response)?)
                    }
                    Err(_) => Err("Intelligent Office authentication failed.".to_string()),
                }
            }
            ClientId::DynamoDbId => {
                Ok(MainContactAddress {
                    address_line_one: "Dynamo Street".to_string(),
                    address_line_two: Some("Suite 101".to_string()),
                    address_line_three: None,
                    address_line_four: None,
                    town_or_city: "Dynamo Town".to_string(),
                    postcode: "12345".to_string()
                })
            }
        }
    }
}
    



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientInformationRepoResponse {
    IntelligentOffice,
    DynamoDb
}

#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoFindAllError {
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoUpdateError {
    InvalidData(String),
    NotFound,
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    InvalidData(String),
    Unknown(String)
}

#[async_trait]
pub trait Repository<T, U> where T: Entity, U: ClientRepoId   {

    /// Insert the received entity in the persistence system
    async fn create(&self, report: T) -> Result<T, RepoCreateError>;

    /// Find and return one single record from the persistence system
    async fn find_one_report(&self, report: FindReport) -> Result<T, RepoSelectError>;

    /// Find and return all records corresponding to the search criteria from the persistence system
    async fn find_all_reports(&self, report: FindReport) -> Result<Vec<T>, RepoFindAllError>;

    /// Update one single record already present in the persistence system
    async fn update(&self, report: T) -> Result<T, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError>;
    
}

#[async_trait]
pub trait QueryExternalRepository<T> where T: ClientRepoId   {

    // Retrieve clients main contact address from persistence system
    async fn get_clients_main_contact_address(id: T) -> Result<MainContactAddress, String>;
    
}