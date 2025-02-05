use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoupleAnnualReviewReportRecommendationsSection {
    introductory_paragraph: String,
    product_recommendations: HashMap<String, Vec<ProductRecommendationText>>,
    other_advice_areas: HashMap<String, String> 
}

// impl CoupleAnnualReviewReportRecommendationsSection {
//     pub fn new(
//         client_1_first_name: &NameString,
//         client_2_first_name: &NameString,
//         client_1_last_name: &NameString,
//         client_2_last_name: &NameString,
//         couple_advice_areas_products: CoupleAdviceAreasAndProductsDto
//     ) -> Result<CoupleAnnualReviewReportRecommendationsSection, ReportError> {

//         let introductory_paragraph = String::from("This section will present my recommendations for each of your accounts, as well as other advice areas we discussed and those I have subsequently reviewed.");
        

        
//     }
// }


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductRecommendationText {
    product_title: String,
    product_retention_sentence: String,
    tax_wrapper_type_features_paragraphs: String,
    actions: Vec<ProductActionsText>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductActionsText {
    action_title: String,
    action_paragraph: String
}