use serde::{Deserialize, Serialize};

use super::{advice_areas::{AdviceAreaDto, AdviceAreasDto}, product::{ExistingNewJointSingleProductDto, ProductsDto}};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAdviceAreasAndProductsDto {
    client_1: AdviceAreasAndProductsDto,
    client_2: Option<AdviceAreasAndProductsDto>,
    joint: Option<AdviceAreasAndProductsDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdviceAreasAndProductsDto {
    advice_areas: AdviceAreasDto,
    products: Option<ProductsDto>
}