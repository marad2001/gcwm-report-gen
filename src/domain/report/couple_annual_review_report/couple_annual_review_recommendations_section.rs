use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::name_string::NameString; 
use crate::domain::constrained_types::tax_year;
use crate::domain::report::advice_areas::{AdviceArea, OtherAdvice};
use crate::domain::report::investment_holdings::InvestmentPortfolio;
use crate::domain::report::objectives::{self, CoupleObjectivesAnnualReview, ObjectiveType};
use crate::domain::report::product::{AccountOrReferenceNumberType, AccountType, CanBeJointlyOwnedAccountType, ExistingJointlyOwnedProduct, ExistingNewJointSingleProduct, ExistingProduct, NewProduct, PlatformAccountNumberType, ProductRetention, Provider, Providers, RecommendedAction, Replace, SingleContribution};
use crate::domain::report::recommendations_section::{AdviceAreasAndProducts, CoupleAdviceAreasAndProducts};
use crate::domain::report::{advice_areas, ReportError};
use crate::driven::repository::InvestmentPortfoliosRepository;
use crate::driving::data_transfer_object::report_type_data_transfer_object::advice_areas_and_products_dto::CoupleAdviceAreasAndProductsDto;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoupleAnnualReviewReportRecommendationsSection {
    introductory_paragraph: String,
    product_recommendations: HashMap<String, ProductRecommendationsText>,
    other_advice_areas: HashMap<String, Vec<(String, String)>> 
}

impl CoupleAnnualReviewReportRecommendationsSection {
    pub async fn new<R>(
        client_1_first_name: &NameString,
        client_2_first_name: &NameString,
        client_1_last_name: &NameString,
        client_2_last_name: &NameString,
        unvalidated_couple_advice_areas_products: CoupleAdviceAreasAndProductsDto,
        objectives: &CoupleObjectivesAnnualReview,
        repo: &R
    ) -> Result<CoupleAnnualReviewReportRecommendationsSection, (String, String)> where R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync  {

        let error_section_string = "Recommendations".to_string();

        if client_1_first_name.to_string() == client_2_first_name.to_string() && client_1_last_name.to_string() == client_2_last_name.to_string() {
            return Err((error_section_string, "Both first and last names are the same for the couple, this looks to be an error".to_string()))
        }

        let objectives_by_id = objectives.objectives_by_id();
        let introductory_paragraph = String::from("This section will present my recommendations for each of your accounts, as well as other advice areas we discussed and those I have subsequently reviewed.");
        let validated_couple_advice_areas_and_products = CoupleAdviceAreasAndProducts::from_dto(unvalidated_couple_advice_areas_products, repo).await.map_err(|error| (error_section_string.to_string(), error))?;
        let client_1_advice_area_products = validated_couple_advice_areas_and_products.client_1;
        let client_2_advice_area_products = validated_couple_advice_areas_and_products.client_2;
        let joint_advice_area_products = validated_couple_advice_areas_and_products.joint;

        let mut product_recommendations_text_by_client = HashMap::new();
        let mut other_advice_areas = HashMap::new();

        let client_1_key = format!("{} {}", client_1_first_name, client_1_last_name);
        let client_2_key = format!("{} {}", client_2_first_name, client_2_last_name);

        create_other_advice_areas(&mut other_advice_areas, &client_1_key, &client_1_advice_area_products);
        create_other_advice_areas(&mut other_advice_areas, &client_2_key, &client_2_advice_area_products);
        create_other_advice_areas(&mut other_advice_areas, "Joint", &joint_advice_area_products);

        create_product_recommendations(
            &mut product_recommendations_text_by_client, 
            &client_1_key, 
            &client_1_advice_area_products,
            &objectives_by_id
        )
            .map_err(|error| (error_section_string.clone(), error))?;
        
        create_product_recommendations(
            &mut product_recommendations_text_by_client, 
            &client_2_key, 
            &client_2_advice_area_products,
            &objectives_by_id
        )
            .map_err(|error| (error_section_string.clone(), error))?;

        create_product_recommendations(
            &mut product_recommendations_text_by_client, 
            "Joint", 
            &joint_advice_area_products,
            &objectives_by_id
        )
            .map_err(|error| (error_section_string.clone(), error))?;
        

        Ok(Self{
            introductory_paragraph,
            product_recommendations: product_recommendations_text_by_client,
            other_advice_areas
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductRecommendationsText {
   existing: Option<Vec<ExistingProductRecommendationText>>,
   new: Option<Vec<NewProductRecommendationText>> 
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExistingProductRecommendationText {
    platform_number: Option<PlatformAccountNumberType>,
    account_or_refence_number: AccountOrReferenceNumberType,
    product_title: String,
    product_retention_sentence: String,
    rationale: String,
    actions: Option<Vec<ProductActionsText>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewProductRecommendationText {
    platform_number: Option<PlatformAccountNumberType>,
    product_title: String,
    new_product_initial_sentence: String,
    rationale: String,
    //actions: Vec<ProductActionsText>
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
    advice_areas_and_products: &Option<AdviceAreasAndProducts>,
    objectives: &HashMap<String, ObjectiveType>
) -> Result<(), String> {
    if let Some(advice_areas_and_products) = advice_areas_and_products {
        if let Some(products) = &advice_areas_and_products.products {
            
            let all_products_by_account_number = products.products_by_account_number_or_new_product_id();
            let existing_products = products.existing_products();
            let new_products = products.new_products();

            // Try collecting all existing product recommendations text, returning an error if any fail
            let existing_product_recommendations: Vec<ProductRecommendationText> = create_existing_products_recommendation_text(
                &all_products_by_account_number, 
                &existing_products,
                objectives
            )?;

            // Insert recommendations only if successful
            product_recommendations
                .entry(client_key.to_string())
                .or_insert_with(Vec::new)
                .extend(existing_product_recommendations);

            let new_product_recommendations: Vec<ProductRecommendationText> = create_new_products_recommendation_text(
                &new_products
            )?;

            product_recommendations
                .entry(client_key.to_string())
                .or_insert_with(Vec::new)
                .extend(new_product_recommendations)
            
        }
    }
    
    Ok(())
}


fn create_existing_products_recommendation_text(
    all_products_by_account_number: &HashMap<String, &ExistingNewJointSingleProduct>,
    existing_products: &Vec<ExistingProduct>,
    objectives: &HashMap<String, ObjectiveType>
) -> Result<Vec<ProductRecommendationText>, String> {
    existing_products
        .iter()
        .map(|existing_product|create_existing_product_recommendation_text
            (
                &all_products_by_account_number, 
                existing_product,
                objectives
            )
        )
        .collect()
} 

fn create_new_products_recommendation_text(
    new_products: &Vec<NewProduct>,
) -> Result<Vec<ProductRecommendationText>, String> {
    new_products
        .iter()
        .map(|new_product| create_new_product_recommendation_text(
            new_product)
        )
        .collect()
}   

fn create_existing_product_recommendation_text(
    all_products_by_account_number_or_reference_number: &HashMap<String, &ExistingNewJointSingleProduct>,
    existing_product_requiring_text: &ExistingProduct,
    objectives_by_id: &HashMap<String, ObjectiveType>
) -> Result<ProductRecommendationText, String> {
    Ok(ProductRecommendationText::ExistingProductRecommendationsText( ExistingProductRecommendationText {
        platform_number: existing_product_requiring_text.platform_account_number().clone(),
        account_or_refence_number: existing_product_requiring_text.account_or_reference_number().clone(),
        product_title: create_existing_product_recommendation_text_title(existing_product_requiring_text),
        product_retention_sentence: create_product_retention_sentence(all_products_by_account_number_or_reference_number, existing_product_requiring_text)?,
        rationale: existing_product_requiring_text.rationale().to_string(),
        actions: create_existing_product_actions_text(
            all_products_by_account_number_or_reference_number, 
            existing_product_requiring_text,
            objectives_by_id
        )?
    }))
}

fn create_new_product_recommendation_text(
    new_product_requiring_text: &NewProduct
) -> Result<ProductRecommendationText, String> {
    Ok(ProductRecommendationText::NewProductRecommendationsText( NewProductRecommendationText {
        platform_number: new_product_requiring_text.platform_account_number().clone(),
        product_title: create_new_product_recommendation_title_text(new_product_requiring_text),
        new_product_initial_sentence: create_new_product_initial_sentence(new_product_requiring_text),
        rationale: new_product_requiring_text.rationale().to_string()
    }))
}

fn create_existing_product_recommendation_text_title(
    product: &ExistingProduct, 
) -> String {

    match product.provider().value() {
        Providers::Transact => {
            format!(
                "{} - {}", 
                product.provider_as_string(), 
                product.account_type_as_string()
            )
        }
        _ => {
            format!(
                "{} - {} - {}", 
                product.provider_as_string(), 
                product.account_type_as_string(),
                product.account_or_reference_number_as_string()
            )
        }
    }
}

fn create_new_product_recommendation_title_text(
    product: &NewProduct
) -> String {

    match product.provider().value() {
        _ => {
            format!(
                "{} - {}",
                product.provider_as_string(),
                product.tax_wrapper_type_as_string()
            )
        }
    }
}

fn create_product_retention_sentence(
    all_products_by_account_number: &HashMap<String, &ExistingNewJointSingleProduct>,
    existing_product: &ExistingProduct
) -> Result<String, String> {

    match existing_product.product_retention() {
        ProductRetention::Retain(_) => {
            Ok(format!(
                "I recommend you continue to retain the {} {}.",
                existing_product.provider().value().alt_name(), 
                existing_product.account_type_as_full_name_brackets_string_short_name(),
            ))
        }
        ProductRetention::FullyEncash(_) => {
            Ok(format!(
                "I recommend you fully encash the {} {}.",
                existing_product.provider().value().alt_name(), 
                existing_product.account_type_as_full_name_brackets_string_short_name()
            ))
        }
        ProductRetention::Replace(replace) => {
           match replace {
                Replace::FullyReplace(fully_replace) => {
                    let fully_replace_to_account_number = fully_replace.replace_to_details().transfer_to_account_or_reference_number();
                    let product_to_be_transferred_to = all_products_by_account_number.get(&fully_replace_to_account_number.to_string());

                    let start_text = format!(
                        "I recommend you transfer in full the {} {} to ",
                        existing_product.provider().value().alt_name(), 
                        existing_product.account_type_as_string()
                    );

                    match product_to_be_transferred_to {
                        Some(product) => {
                            let transfer_text = match product {
                                ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(transfer_to_existing_product) => {
                                    format!(
                                        "the existing jointly owned {} {}.",
                                        transfer_to_existing_product.provider().value().alt_name(),
                                        transfer_to_existing_product.tax_wrapper_type_as_string()
                                    )
                                },
                                ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(transfer_to_existing_product) => {
                                    format!(
                                        "your existing {} {}.",
                                        transfer_to_existing_product.provider().value().alt_name(),
                                        transfer_to_existing_product.tax_wrapper_type_as_string()
                                    )
                                },
                                ExistingNewJointSingleProduct::NewSingleOwnedProduct(new_product) => {
                                    format!(
                                        "a new {} {}.",
                                        new_product.provider().value().alt_name(),
                                        new_product.tax_wrapper_type_as_string()
                                    )
                                }
                            };
                            Ok(format!("{}{}", start_text, transfer_text))
                        }
                        None => {
                            Err(format!(
                                "No matching recommended product to be transferred to found for the product being fully replaced. Account or reference number provided was {}",
                                fully_replace_to_account_number.to_string()
                            ))
                        }
                    }
                }
                Replace::PartiallyReplace(partially_replace) => {
                    let partially_replace_to_account_number = partially_replace.partially_replace_to_details().transfer_to_account_or_reference_number();
                    let product_to_be_transferred_to = all_products_by_account_number.get(&partially_replace_to_account_number.to_string());

                    let start_text = format!(
                        "I recommend you partially transfer the {} {} to ",
                        existing_product.provider().value().alt_name(), 
                        existing_product.account_type_as_string()
                    );

                    match product_to_be_transferred_to {
                        Some(product) => {
                            let transfer_text = match product {
                                ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(transfer_to_existing_product) => {
                                    format!(
                                        "the existing jointly owned {} {}.",
                                        transfer_to_existing_product.provider().value().alt_name(),
                                        transfer_to_existing_product.tax_wrapper_type_as_string()
                                    )
                                },
                                ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(transfer_to_existing_product) => {
                                    format!(
                                        "your existing {} {}.",
                                        transfer_to_existing_product.provider().value().alt_name(),
                                        transfer_to_existing_product.tax_wrapper_type_as_string()
                                    )
                                },
                                ExistingNewJointSingleProduct::NewSingleOwnedProduct(new_product) => {
                                    format!(
                                        "a new {} {}.",
                                        new_product.provider().value().alt_name(),
                                        new_product.tax_wrapper_type_as_string()
                                    )
                                }
                            };
                            Ok(format!("{}{}", start_text, transfer_text))
                        }
                        None => {
                            Err(format!(
                                "No matching recommended product to be transferred to found for the product being partially replaced. Account or reference number provided was {}",
                                partially_replace_to_account_number.to_string()
                            ))
                        }
                    }
                }
            }
        }
    }
}



fn create_new_product_initial_sentence(
    new_product_requiring_text: &NewProduct
) -> String {

    match new_product_requiring_text.provider().value() {
        Providers::Abrdn => {
            match new_product_requiring_text.platform_account_number() {
                Some(_) => {
                    format!(
                        "I recommend you open a new {} within your existing {} wrap account.",
                        new_product_requiring_text.tax_wrapper_type_as_string(),
                        new_product_requiring_text.provider_as_string()
                    )
                },
                None => {
                    format!(
                        "I recommend you open a new {} wrap account and within this account open a {}",
                        new_product_requiring_text.provider_as_string(),
                        new_product_requiring_text.tax_wrapper_type_as_string()
                    )
                }
            }
        }
        _ => {
            match new_product_requiring_text.platform_account_number() {
                Some(_) => {
                    format!(
                        "I recommend you open a new {} within your existing {} platform account.",
                        new_product_requiring_text.tax_wrapper_type_as_string(),
                        new_product_requiring_text.provider_as_string()
                    )
                },
                None => {
                    format!(
                        "I recommend you open a new {} platform account and within this account open a {}",
                        new_product_requiring_text.provider_as_string(),
                        new_product_requiring_text.tax_wrapper_type_as_string()
                    )
                }
            }
        }
    }

}


fn create_existing_product_actions_text(
    all_products_by_account_number_or_reference_number: &HashMap<String, &ExistingNewJointSingleProduct>, 
    existing_product_requiring_text: &ExistingProduct,
    objectives_by_id: &HashMap<String, ObjectiveType>
) -> Result<Option<Vec<ProductActionsText>>, String> {
    // We only support SingleOwned products for now.
    if let ExistingProduct::SingleOwned(existing_product) = existing_product_requiring_text {
        // Check if the product retention is of type Retain.
        if let ProductRetention::Retain(retention_recommendations) = existing_product.product_retention() {
            // Check if there are recommendation actions.
            if let Some(recommended_actions) = retention_recommendations.recommendation_actions() {
                // For each recommended action, call create_action_paragraph (which returns a Result)
                let actions: Result<Vec<ProductActionsText>, String> = recommended_actions
                    .iter()
                    .map(|action| {
                        let paragraph = create_action_paragraph(
                            all_products_by_account_number_or_reference_number, 
                            existing_product_requiring_text, 
                            action,
                            objectives_by_id
                        )?;
                        Ok(ProductActionsText {
                            action_title: action.description().to_string(),
                            action_paragraph: paragraph,
                        })
                    })
                    .collect();
                return Ok(Some(actions?));
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
    }
    Ok(None)
}



fn create_action_paragraph(
    all_products_by_account_number_or_reference_number: &HashMap<String, &ExistingNewJointSingleProduct>, 
    existing_product_requiring_text: &ExistingProduct,
    recommended_action: &RecommendedAction,
    objectives: &HashMap<String, ObjectiveType>
) -> Result<String, String> {
    let mut action_paragraph = String::new();

    match recommended_action {
        RecommendedAction::SingleContribution(single_contribution) => {
            
            // Create the first sentence
            if single_contribution.tax_year_of_action().is_some() {
                action_paragraph.push_str(format!(
                    "I recommend you make a contribution of {} in the {} tax year.",
                    single_contribution.value(),
                    single_contribution.tax_year_of_action().unwrap()
                ).as_str())
            } else if single_contribution.date_of_action().is_some() {
                action_paragraph.push_str(format!(
                    "I recommend you make a contribution of {} on the {}.",
                    single_contribution.value(),
                    single_contribution.date_of_action().unwrap()
                ).as_str());
            } else if single_contribution.date_of_action().is_some() && single_contribution.tax_year_of_action().is_some() {
                action_paragraph.push_str(format!(
                    "I recommend you make a contribution of {} on the {} in the {} tax year.",
                    single_contribution.value(),
                    single_contribution.date_of_action().unwrap(),
                    single_contribution.tax_year_of_action().unwrap()
                ).as_str());
            } else {    
                action_paragraph.push_str(format!(
                    "I recommend you make a contribution of {}.",
                    single_contribution.value()
                ).as_str());
            }

            // Create the rationale
            // if single_contribution.rationale().is_none() {
            //     match existing_product_requiring_text {
            //         ExistingProduct::JointlyOwned(jointly_owned) => {
            //             match jointly_owned.account_type() {
            //                 CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(_) => {
            //                     Err("We would never recommend a contribution is made into a Joint General Investment Account".to_string());
            //                 },
            //                 CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(_) => {
            //                     let linked_objectives = existing_product_requiring_text.linked_objectives();
            //                     for objective_id in linked_objectives {
            //                         match objectives.get(objective_id) {
            //                             Some(objective) => {
            //                                 match objective {
            //                                     ObjectiveType::CapitalProtectionObjective(cap_pro_obj) => {

            //                                     }
            //                                     ObjectiveType::CoupleIncomeObjective(coup_inc_obj) => {

            //                                     }
            //                                     ObjectiveType::IhtObjective(iht_obj) => {

            //                                     }
            //                                     ObjectiveType::InRetirementIncomeObjective(in_ret_inc_obj) => {

            //                                     }
            //                                     ObjectiveType::IncomeObjective(inc_obj) => {

            //                                     }
            //                                     ObjectiveType::OtherObjective(other_obj) => {

            //                                     }
            //                                 }
            //                             }
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //         ExistingProduct::SingleOwned(single_owned) => {
            //             match single_owned.account_type() {
            //                 AccountType::GeneralInvestmentAccount(_) => {

            //                 }
            //                 AccountType::IsaStocksAndShares(_) => {

            //                 }
            //                 AccountType::PersonalPension(_) | AccountType::SelfInvestedPersonalPension(_) => {

            //                 }
            //                 AccountType::OffshoreInvestmentBond(_) => {
                                
            //                 }
            //                 AccountType::OnshoreInvestmentBond(_) => {

            //                 }
            //             }
            //         }
            //     }
            // }
            
        
                
               
                    
                
            
            
            
        }
        _ => {}
    }
    
    Ok(action_paragraph.to_string())

}