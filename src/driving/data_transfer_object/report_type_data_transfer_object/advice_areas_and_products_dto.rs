use serde::{Deserialize, Serialize};

use super::{advice_areas::{AdviceAreaDto, AdviceAreasDto}, product::{ExistingNewJointSingleProductDto, ProductsDto}};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAdviceAreasAndProductsDto {
    pub client_1: Option<AdviceAreasAndProductsDto>,
    pub client_2: Option<AdviceAreasAndProductsDto>,
    pub joint: Option<AdviceAreasAndProductsDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdviceAreasAndProductsDto {
    pub advice_areas: Option<AdviceAreasDto>,
    pub products: Option<ProductsDto>
}