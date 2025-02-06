use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::name_string::NameString; 
use crate::domain::report::advice_areas::{AdviceArea, OtherAdvice};
use crate::domain::report::product::{self, ExistingJointlyOwnedProduct, ExistingNewJointSingleProduct, ExistingProduct, ProductRetention, Providers};
use crate::domain::report::recommendations_section::{AdviceAreasAndProducts, CoupleAdviceAreasAndProducts};
use crate::domain::report::{advice_areas, ReportError};
use crate::driving::data_transfer_object::report_type_data_transfer_object::advice_areas_and_products_dto::CoupleAdviceAreasAndProductsDto;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoupleAnnualReviewReportRecommendationsSection {
    introductory_paragraph: String,
    product_recommendations: HashMap<String, Vec<ProductRecommendationText>>,
    other_advice_areas: HashMap<String, Vec<(String, String)>> 
}

impl CoupleAnnualReviewReportRecommendationsSection {
    pub fn new(
        client_1_first_name: &NameString,
        client_2_first_name: &NameString,
        client_1_last_name: &NameString,
        client_2_last_name: &NameString,
        unvalidated_couple_advice_areas_products: CoupleAdviceAreasAndProductsDto
    ) -> Result<CoupleAnnualReviewReportRecommendationsSection, (String, String)> {

        let error_section = "Recommendations".to_string();
        let introductory_paragraph = String::from("This section will present my recommendations for each of your accounts, as well as other advice areas we discussed and those I have subsequently reviewed.");
        let validated_couple_advice_areas_and_products = CoupleAdviceAreasAndProducts::try_from(unvalidated_couple_advice_areas_products).map_err(|error| (error_section.to_string(), error))?;
        let client_1_advice_area_products = validated_couple_advice_areas_and_products.client_1;
        let client_2_advice_area_products = validated_couple_advice_areas_and_products.client_2;
        let joint_advice_area_products = validated_couple_advice_areas_and_products.joint;

        let mut product_recommendations = HashMap::new();
        let mut other_advice_areas = HashMap::new();

        let client_1_key = format!("{} {}", client_1_first_name, client_1_last_name);
        let client_2_key = format!("{} {}", client_2_first_name, client_2_last_name);

        create_other_advice_areas(&mut other_advice_areas, &client_1_key, &client_1_advice_area_products);
        create_other_advice_areas(&mut other_advice_areas, &client_2_key, &client_2_advice_area_products);
        create_other_advice_areas(&mut other_advice_areas, "Joint", &joint_advice_area_products);

        create_product_recommendations(&mut product_recommendations, &client_1_key, &client_1_advice_area_products);
        

        Ok(Self{
            introductory_paragraph,
            product_recommendations,
            other_advice_areas
        })
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExistingProductRecommendationText {
    product_title: String,
    product_retention_sentence: String,
    rationale: String,
    actions: Vec<ProductActionsText>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductActionsText {
    action_title: String,
    action_paragraph: String
}

fn create_other_advice_areas(
    other_advice_areas: &mut HashMap<String, Vec<(String, String)>>,
    key: &str,
    advice_areas_and_products: &Option<AdviceAreasAndProducts>
) {
    if let Some(advice_areas_and_products) = advice_areas_and_products {
        if let Some(advice_areas) = &advice_areas_and_products.advice_areas {
            for advice_area in advice_areas.value() {
                other_advice_areas
                    .entry(key.to_string())
                    .or_insert_with(Vec::new)
                    .push(create_other_advice_area(advice_area));
            }
        }
    }
}


fn create_other_advice_area(advice_area: &AdviceArea) -> (String, String) {
    match advice_area {
        AdviceArea::EmergencyFund(emergency_fund_advice) => (advice_area.title(), emergency_fund_advice.advice.to_string()),
        AdviceArea::Iht(iht_advice) => (advice_area.title(), iht_advice.advice.to_string()),
        AdviceArea::Other(other_advice) => (advice_area.title(), other_advice.advice.to_string()),
        AdviceArea::Will(will_advice) => (advice_area.title(), will_advice.advice.to_string()),
        AdviceArea::Poa(poa_advice) => (advice_area.title(), poa_advice.advice.to_string())
    }
}

fn create_product_recommendations(
    product_recommendations: &mut HashMap<String, Vec<ProductRecommendationText>>,
    client_key: &str,
    advice_areas_and_products: &Option<AdviceAreasAndProducts>
) {
    if let Some(advice_areas_and_products) = advice_areas_and_products {
        if let Some(products) = &advice_areas_and_products.products {
            
            let existing_products = products.existing_products();
            let new_products = products.new_products();
            
            for product in existing_products {
                product_recommendations
                    .entry(client_key.to_string())
                    .or_insert_with(Vec::new)
                    .push(create_existing_product_recommendation_text(product, client_key))
            }
        }
    }
}

fn create_existing_products_recommendation_text(
    products: Vec<&ExistingProduct>,
) -> Vec<ExistingProductRecommendationText> {
    products
        .iter()
        .map(|product|create_existing_product_recommendation_text(product))
        .collect()
} 

fn create_existing_product_recommendation_text(
    product: &ExistingProduct,
) -> ExistingProductRecommendationText {
    ExistingProductRecommendationText {
        product_title: create_existing_product_recommendation_text_title(product),
        product_retention_sentence: create_product_retention_paragraph(product)
    }
}

fn create_existing_product_recommendation_text_title(
    product: &ExistingProduct, 
) -> String {

    match product.provider().value() {
        Providers::Transact => {
            format!(
                "{} - {}", 
                product.provider_as_string(), 
                product.tax_wrapper_type_as_string()
            )
        }
        _ => {
            format!(
                "{} - {} - {}", 
                product.provider_as_string(), 
                product.tax_wrapper_type_as_string(),
                product.account_or_reference_number_as_string()
            )
        }
    }
}

fn create_product_retention_paragraph(product: &ExistingProduct) -> String {
    
    match product.product_retention() {
        ProductRetention::FullyEncash(_) => {
            format!(
                "I recommend you fully encash the {} {}.",
                product.provider().alt_name(), 
                product.tax_wrapper_type_as_string_short_name() 
            )
        },
        ProductRetention::Replace(_) => {
            format!(
                "I recommend you replace the {} {}.",
                product.provider().alt_name(), 
                product.tax_wrapper_type_as_string_short_name() 
            )
        }
        ProductRetention::PartialTransfer(_) => {
            format!(
                "I recommend you partially transfer the {} {}.",
                product.provider().alt_name(), 
                product.tax_wrapper_type_as_string_short_name() 
            )
        }
        ProductRetention::Retain(_) => {
            format!(
                "I recommend you continue to retain the {} {}."
            )
        }
    }
}