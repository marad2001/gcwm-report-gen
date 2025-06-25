use std::num::NonZeroU128;

use uuid::Uuid;

use crate::{domain::{find_model_portfolio::find_one_model_portfolio, report::investment_holdings::InvestmentPortfolio}, driven::repository::InvestmentPortfoliosRepository, driving::data_transfer_object::{
    report_type_data_transfer_object::{
        advice_areas::{AdviceAreaDto, AdviceAreasDto, EmergencyFundAdvice, IhtAdvice, PoaAdvice, WillAdvice}, advice_areas_and_products_dto::{AdviceAreasAndProductsDto, CoupleAdviceAreasAndProductsDto}, adviser_data_transfer_object::AdviserDataTransferObject, background_section_data_transfer_objects::{
            AdditionalCompanyMeetingAttendeeDataTransferObject, 
            HomeMeetingLocationDataTransferObject, 
            MeetingLocationDataTransferObject
        }, couple_annual_review_data_transfer_object::{
            couple_annual_review_report_background_section_dto::CoupleAnnualReviewBackgroundSectionDataTransferObject, couple_annual_review_report_current_circumstances_section_dto::CoupleAnnualReviewReportCurrentCircumstancesSectionDto, couple_annual_review_report_sections_data_transfer_object::CoupleAnnualReviewReportSectionsDataTransferObject, CoupleAnnualReviewReportDataTransferObject
        }, current_circumstances_section_dto::{
            ChangeInCircumstancesDto, CoupleIsChangeRiskToleranceDto, IsChangeInCircumstancesDto, IsChangeRiskToleranceDto
        }, investment_holdings::{
            FundHoldingDto, InvestmentPortfolioDto, InvestmentStrategyDto, InvestmentStrategyProductTypeDto, InvestmentStrategyProviderDto, InvestmentStrategyServicePropositionDto, ModelPortfolioIdDto, MonthYearDto, VersionedPortfolioDto
        }, objectives_dto::{
            ChangeInObjectivesDto, ClientFromAgeDto, CoupleIncomeObjectiveDto, CoupleObjectivesAnnualReviewDto, IncomeObjectiveDto, ObjectiveTypeDto
        }, product::{
            AccountOrReferenceNumberTypeDto, AccountTypeDto, BespokeInvestmentStrategyDto, BespokeOrFirmInvestmentStrategyDto, ExistingNewJointSingleProductDto, ExistingProductRecommendationsDto, ExistingSingleOwnedProductDto, IsaStocksAndSharesDto, KnownOrUnknownAccountDto, OngoingChargeDto, OtherChargeDto, PlatformAccountNumberTypeDto, PresentFirmInvestmentStrategyDto, ProductChargesDto, ProductRetentionDto, ProductsDto, ProviderDto, ProvidersDto, RealignOrRebalanceDto, RebalanceDto, RecommendedActionDto, RecommendedInvestmentAndRiskStrategyDto, RetainDto, SelfInvestedPersonalPensionDto, SingleContributionDto, TransferDetailDto, TransferDto, ValuationDto
        }, risk_assessment_dto::RiskProfileDto, ReportTypeDataTransferObject
    }, 
    DataTransferObject
}};


// pub fn create_mock_data_transfer_object() -> DataTransferObject {
//     DataTransferObject {
//         report_type: ReportTypeDataTransferObject::CoupleAnnualReviewReportDataTransferObject(
//             CoupleAnnualReviewReportDataTransferObject {
//                 individual_one_first_name: "Terry".to_string(),
//                 individual_one_last_name: "Tester".to_string(),
//                 individual_two_first_name: "Tessa".to_string(),
//                 individual_two_last_name: "Tester".to_string(),
//                 adviser: AdviserDataTransferObject {
//                     adviser_first_name: "Martin".to_string(),
//                     adviser_last_name: "Blocks".to_string()
//                 },
//                 sections: CoupleAnnualReviewReportSectionsDataTransferObject {
//                     background: CoupleAnnualReviewBackgroundSectionDataTransferObject {
//                         meeting_location: MeetingLocationDataTransferObject::Home(
//                             HomeMeetingLocationDataTransferObject {
//                                 town: "Charmouth".to_string()
//                             }
//                         ),
//                         additional_company_attendees: Some(vec![
//                             AdditionalCompanyMeetingAttendeeDataTransferObject {
//                                 first_name: "Adam".to_string(),
//                                 last_name: "Free".to_string()
//                             }
//                         ]),
//                         additional_attendees: None,
//                         meeting_date: "07/08/2024".to_string()
//                     },
//                     current_circumstances: CoupleAnnualReviewReportCurrentCircumstancesSectionDto {
//                         last_meeting_date: "07/02/2022".to_string(),
//                         last_review_report_date: "28/07/2023".to_string(),
//                         is_change_in_circumstances: IsChangeInCircumstancesDto::ChangeInCircumstances(
//                             ChangeInCircumstancesDto {
//                                 circumstances: vec![
//                                     "You have recently returned to the UK after 9 months of travelling through. During this period, you let out your main residence, on a long-term tenancy to an individual who was considering the purchase of the property and the surrounding land, dependent on the outcome of a planning application made with the Local Authority.".to_string(),
//                                     "You recently found out that the individual would like to exercise their right to purchase the property and you have recently exchanged on the property with completion set for the beginning of the new financial year on the 6th April 2025.".to_string(),
//                                     "Since our last review, you have become grandparents following the birth of first child, who was born in June 2024. You are interested in setting up some sort of investment for and we discussed Junior ISAs during our meeting. I have added detail around this later in this report.".to_string(),
//                                     "You both continue to several days a week at the premises on. You are no longer heavily involved in the day-to-day management of the business as this is dealt with by the manager.".to_string(),
//                                     "Your plan over the next 10 years or so is largely dependent on the sale of as this will determine your next project. You have a few options including the development of land that owns in, converting the use of a field that you own in, converting the use of a field that you own or consider applying for planning on the to build a residential property.".to_string(),
//                                     "You plan to experience further trips in the future similar to your recent trip to.".to_string(),
//                                     "You provided me with updated rental incomes received and the investment properties that you own.".to_string(),
//                                     "We clarified your approximate spending target and agreed that circa £40,000.00 is spent on ‘essential’ spending each year (i.e., household bills and food) and we estimated that £60,000.00 is likely to be spent on discretionary spending each year.".to_string(),
//                                     "We also discussed the pension that Tessa owns that used to be administered by. We were unable to locate the administrators of this scheme in recent years, until a statement was found from. Tessa, you signed a letter of authority to allow us to obtain information on this plan. We are still awaiting this pension information, and shall give full advice in another report at a later date.".to_string(),
//                                     "Terry, you have utilised your ISA Allowance for the current tax year by adding £20,000.00 into a Cash ISA. Tessa, you have not yet utilised your ISA Allowance for the tax year.".to_string()
//                                 ]
//                             }
//                         ),
//                         couple_objectives: CoupleObjectivesAnnualReviewDto {
//                             client_1_objectives: None,
//                             client_2_objectives: None,
//                             shared_objectives: Some(ChangeInObjectivesDto::NoChangeInObjectives(
//                                 vec![
//                                     ObjectiveTypeDto::CoupleIncomeObjective(
//                                         CoupleIncomeObjectiveDto {
//                                             id: Uuid::parse_str("7f88927c-6da3-429e-aa7a-8600d94399e6").unwrap(),
//                                             annual_income: 100000.00,
//                                             frequency: "Annually".to_string(),
//                                             from_year: None,
//                                             from_age: Some(ClientFromAgeDto::Client1(60)),
//                                             linked_risk_profile: RiskProfileDto::Moderate
//                                         }
//                                     )
//                                 ]
//                             ))
//                         },
//                         couple_is_risk_tolerance_change: CoupleIsChangeRiskToleranceDto {
//                             client_1: IsChangeRiskToleranceDto::NoChangeRiskTolerance(RiskProfileDto::Moderate),
//                             client_2: IsChangeRiskToleranceDto::NoChangeRiskTolerance(RiskProfileDto::Moderate)
//                         }
//                     },
//                     recommendations: CoupleAdviceAreasAndProductsDto {
//                         client_1: Some(AdviceAreasAndProductsDto {
//                             advice_areas: None,
//                             products: Some(ProductsDto::new(vec![
//                                 ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(
//                                     ExistingSingleOwnedProductDto {
//                                         id: "9ce60e0f-becf-40b0-8749-84afc8a4a1df".to_string(),
//                                         platform_or_account_number: Some(PlatformAccountNumberTypeDto::Transact("412-324-898".to_string())),
//                                         account_or_reference_number: AccountOrReferenceNumberTypeDto::Transact("IH00408765".to_string()),
//                                         account_type: AccountTypeDto::IsaStocksAndShares(
//                                             IsaStocksAndSharesDto {
//                                                 provider: ProviderDto::new(ProvidersDto::Transact),
//                                                 optional_description: None,
//                                                 current_investment_strategy: InvestmentStrategyDto::Model(
//                                                     VersionedPortfolioDto {
//                                                         id: ModelPortfolioIdDto {
//                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                             sri: false,
//                                                             risk_profile: RiskProfileDto::Moderate,
//                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                         },
//                                                         effective_date: MonthYearDto::Aug24,
//                                                     }
//                                                 ),
//                                                 current_value: ValuationDto {
//                                                     value: 29605.25,
//                                                     date_of_valuation: "11/02/2025".to_string()
//                                                 },
//                                                 linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto::Transact("IH00408765".to_string()),
//                                                 charges: ProductChargesDto {
//                                                     ongoing_advice_charge: 0.50,
//                                                     platform_charge: 0.26,
//                                                     ongoing_fund_charge: Some(0.63),
//                                                     other_charges: Some(OtherChargeDto {
//                                                         ongoing_charges: Some(vec![
//                                                             OngoingChargeDto {
//                                                                 charge_description: "Administration Wrapper Charge".to_string(),
//                                                                 charge_value: 12.00,
//                                                                 frequency: "Annually".to_string(),
//                                                             }
//                                                         ]),
//                                                         incidental_charges: None
//                                                     })
//                                                 },
//                                                 recommendations: ExistingProductRecommendationsDto {
//                                                     product_retention: ProductRetentionDto::Retain(RetainDto {
//                                                         rationale: "ISAs offer the potential for capital growth in a tax-efficient manner as any income produced by the underlying funds is not liable to Income Tax and any gains realised are not subject to Capital Gains Tax (CGT). These taxes will therefore not act as a drag on investment returns which will aid in achieving your objectives.".to_string(),
//                                                         recommended_product_charges: ProductChargesDto {
//                                                             ongoing_advice_charge: 0.72,
//                                                             platform_charge: 0.26,
//                                                             ongoing_fund_charge: None,
//                                                             other_charges: Some(OtherChargeDto {
//                                                                 ongoing_charges: Some(vec![
//                                                                     OngoingChargeDto {
//                                                                         charge_description: "Administration Wrapper Charge".to_string(),
//                                                                         charge_value: 12.00,
//                                                                         frequency: "Annually".to_string(),
//                                                                     }
//                                                                 ]),
//                                                                 incidental_charges: None
//                                                             })
//                                                         },
//                                                         recommended_investment_strategy: RealignOrRebalanceDto::Rebalance(RebalanceDto {
//                                                             rationale: "The existing fund and asset allocation selection still remains suitable and as such I recommend rebalancing the portfolio to avoid portfolio drift which can result in your investment falling out of line with your risk profile.".to_string(),
//                                                             recommended_investment_strategy: InvestmentStrategyDto::Model(
//                                                                 VersionedPortfolioDto {
//                                                                     id: ModelPortfolioIdDto {
//                                                                         provider: InvestmentStrategyProviderDto::Transact,
//                                                                         service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                                         sri: false,
//                                                                         risk_profile: RiskProfileDto::Moderate,
//                                                                         product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                                     },
//                                                                     effective_date: MonthYearDto::Aug24,
//                                                                 }
//                                                             ),
//                                                         }),
//                                                         linked_objectives: vec!["7f88927c-6da3-429e-aa7a-8600d94399e6".to_string()],
//                                                         recommendation_actions: None
//                                                     })
//                                                 }
//                                             }
//                                         )
//                                     }
//                                 ),
//                                 ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(
//                                     ExistingSingleOwnedProductDto {
//                                         id: "59cb8ae4-bec0-4112-8727-433e5a4371bb".to_string(),
//                                         platform_or_account_number: Some(PlatformAccountNumberTypeDto::Transact("568-856-757".to_string())),
//                                         account_or_reference_number: AccountOrReferenceNumberTypeDto::Transact("IH00564856".to_string()),
//                                         account_type: AccountTypeDto::IsaStocksAndShares(
//                                             IsaStocksAndSharesDto {
//                                                 provider: ProviderDto::new(ProvidersDto::Transact),
//                                                 optional_description: Some("New ISA".to_string()),
//                                                 current_investment_strategy: InvestmentStrategyDto::Model(
//                                                     VersionedPortfolioDto {
//                                                         id: ModelPortfolioIdDto {
//                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                             sri: false,
//                                                             risk_profile: RiskProfileDto::Moderate,
//                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                         },
//                                                         effective_date: MonthYearDto::Aug24,
//                                                     }
//                                                 ),
//                                                 current_value: ValuationDto { value: 67347.83, date_of_valuation: "11/02/2025".to_string() },
//                                                 linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto::Transact("IH00564856".to_string()),
//                                                 charges: ProductChargesDto {
//                                                     ongoing_advice_charge: 0.72,
//                                                     platform_charge: 0.26,
//                                                     ongoing_fund_charge: Some(0.45),
//                                                     other_charges: Some(OtherChargeDto {
//                                                         ongoing_charges: Some(vec![
//                                                             OngoingChargeDto {
//                                                                 charge_description: "Administration Wrapper Charge".to_string(),
//                                                                 charge_value: 12.00,
//                                                                 frequency: "Annually".to_string(),
//                                                             }
//                                                         ]),
//                                                         incidental_charges: None
//                                                     })
//                                                 },
//                                                 recommendations: ExistingProductRecommendationsDto {
//                                                     product_retention: ProductRetentionDto::Retain(
//                                                         RetainDto { 
//                                                             rationale: "for the same reasons as previously outlined.".to_string(), 
//                                                             recommended_product_charges: ProductChargesDto {
//                                                                 ongoing_advice_charge: 0.72,
//                                                                 platform_charge: 0.26,
//                                                                 ongoing_fund_charge: None,
//                                                                 other_charges: Some(OtherChargeDto {
//                                                                     ongoing_charges: Some(vec![
//                                                                         OngoingChargeDto {
//                                                                             charge_description: "Administration Wrapper Charge".to_string(),
//                                                                             charge_value: 12.00,
//                                                                             frequency: "Annually".to_string(),
//                                                                         }
//                                                                     ]),
//                                                                     incidental_charges: None
//                                                                 })
//                                                             }, 
//                                                             recommended_investment_strategy: RealignOrRebalanceDto::Rebalance(RebalanceDto {
//                                                                 rationale: "The existing fund and asset allocation selection still remains suitable and as such I recommend rebalancing the portfolio to avoid portfolio drift which can result in your investment falling out of line with your risk profile.".to_string(),
//                                                                 recommended_investment_strategy: InvestmentStrategyDto::Model(
//                                                                     VersionedPortfolioDto {
//                                                                         id: ModelPortfolioIdDto {
//                                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                                             sri: false,
//                                                                             risk_profile: RiskProfileDto::Moderate,
//                                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                                         },
//                                                                         effective_date: MonthYearDto::Aug24,
//                                                                     }
//                                                                 ),
//                                                             }),
//                                                             linked_objectives: vec!["7f88927c-6da3-429e-aa7a-8600d94399e6".to_string()], 
//                                                             recommendation_actions: Some(vec![
//                                                                 RecommendedActionDto::Transfer(TransferDto {
//                                                                     id: "3dc7f24e-31d4-4653-9f51-112974075d91".to_string(),
//                                                                     value: 20000.00,
//                                                                     executive_summary_description_receiving_product: Some("Receive the transfer of £20,000.00 from your Fish3 Cash ISA".to_string()),
//                                                                     executive_summary_description_transferring_product: None,
//                                                                     rationale: "Cos this is better for you".to_string(),
//                                                                     date_of_action: None,
//                                                                     tax_year_of_action: None,
//                                                                     transfer_details: TransferDetailDto {
//                                                                         transfer_to_account_or_reference_number: AccountOrReferenceNumberTypeDto::Transact("IH00564856".to_string()),
//                                                                         transfer_from_account_or_reference_number: KnownOrUnknownAccountDto::Unknown{
//                                                                             description: "Fish Cash ISA".to_string(),
//                                                                             account_type: "Cash ISA".to_string()
//                                                                         },
//                                                                     }
//                                                                 })
//                                                             ])
//                                                         }
//                                                     )
//                                                 }
//                                             }
//                                         )
//                                     }
//                                 ),
//                                 ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(
//                                     ExistingSingleOwnedProductDto {
//                                         id: "05995cb8-fefe-4718-a15f-ec3f7c952953".to_string(),
//                                         platform_or_account_number: None,
//                                         account_or_reference_number: AccountOrReferenceNumberTypeDto::Other("WHIX015879".to_string()),
//                                         account_type: AccountTypeDto::IsaStocksAndShares(
//                                             IsaStocksAndSharesDto {
//                                                 provider: ProviderDto::new(ProvidersDto::Fidelity),
//                                                 optional_description: None,
//                                                 current_investment_strategy: InvestmentStrategyDto::Model(
//                                                     VersionedPortfolioDto {
//                                                         id: ModelPortfolioIdDto {
//                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                             sri: false,
//                                                             risk_profile: RiskProfileDto::Moderate,
//                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                         },
//                                                         effective_date: MonthYearDto::Aug24,
//                                                     }
//                                                 ),
//                                                 current_value: ValuationDto { value: 21045.75, date_of_valuation: "11/02/2025".to_string() },
//                                                 linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto::Other("WHIX015879".to_string()),
//                                                 charges: ProductChargesDto {
//                                                     ongoing_advice_charge: 0.69,
//                                                     platform_charge: 0.25,
//                                                     ongoing_fund_charge: Some(0.69),
//                                                     other_charges: None
//                                                 },
//                                                 recommendations: ExistingProductRecommendationsDto {
//                                                     product_retention: ProductRetentionDto::Retain(
//                                                         RetainDto { 
//                                                             rationale: "for the same reasons as previously outlined.".to_string(), 
//                                                             recommended_product_charges: ProductChargesDto {
//                                                                 ongoing_advice_charge: 0.72,
//                                                                 platform_charge: 0.25,
//                                                                 ongoing_fund_charge: None,
//                                                                 other_charges: None
//                                                             }, 
//                                                             recommended_investment_strategy: RealignOrRebalanceDto::Rebalance(RebalanceDto {
//                                                                 rationale: "The existing fund and asset allocation selection still remains suitable and as such I recommend rebalancing the portfolio to avoid portfolio drift which can result in your investment falling out of line with your risk profile.".to_string(),
//                                                                 recommended_investment_strategy: InvestmentStrategyDto::Model(
//                                                                     VersionedPortfolioDto {
//                                                                         id: ModelPortfolioIdDto {
//                                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                                             sri: false,
//                                                                             risk_profile: RiskProfileDto::Moderate,
//                                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                                         },
//                                                                         effective_date: MonthYearDto::Aug24,
//                                                                     }
//                                                                 ),
//                                                             }),
//                                                             linked_objectives: vec!["7f88927c-6da3-429e-aa7a-8600d94399e6".to_string()], 
//                                                             recommendation_actions: None
//                                                         }
//                                                     )
//                                                 }
//                                             }
//                                         )
//                                     }
//                                 ),
//                                 ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(
//                                     ExistingSingleOwnedProductDto {
//                                         id: "0a8b59f0-603d-4d54-b4c5-cc9d008c5a48".to_string(),
//                                         platform_or_account_number: None,
//                                         account_or_reference_number: AccountOrReferenceNumberTypeDto::Other("AC25087900-002".to_string()),
//                                         account_type: AccountTypeDto::IsaStocksAndShares(
//                                             IsaStocksAndSharesDto {
//                                                 provider: ProviderDto::new(ProvidersDto::Quilter),
//                                                 optional_description: None,
//                                                 current_investment_strategy: InvestmentStrategyDto::Model(
//                                                     VersionedPortfolioDto {
//                                                         id: ModelPortfolioIdDto {
//                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                             sri: false,
//                                                             risk_profile: RiskProfileDto::Moderate,
//                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                         },
//                                                         effective_date: MonthYearDto::Aug24,
//                                                     }
//                                                 ),
//                                                 current_value: ValuationDto { value: 76241.58, date_of_valuation: "11/02/2025".to_string() },
//                                                 linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto::Other("AC25087900-002".to_string()),
//                                                 charges: ProductChargesDto {
//                                                     ongoing_advice_charge: 0.00,
//                                                     platform_charge: 0.31,
//                                                     ongoing_fund_charge: Some(0.31),
//                                                     other_charges: None
//                                                 },
//                                                 recommendations: ExistingProductRecommendationsDto {
//                                                     product_retention: ProductRetentionDto::Retain(
//                                                         RetainDto { 
//                                                             rationale: "for the same reasons as previously outlined.".to_string(), 
//                                                             recommended_product_charges: ProductChargesDto {
//                                                                 ongoing_advice_charge: 0.72,
//                                                                 platform_charge: 0.31,
//                                                                 ongoing_fund_charge: None,
//                                                                 other_charges: None
//                                                             }, 
//                                                             recommended_investment_strategy: RealignOrRebalanceDto::Rebalance(RebalanceDto {
//                                                                 rationale: "The existing fund and asset allocation selection still remains suitable and as such I recommend rebalancing the portfolio to avoid portfolio drift which can result in your investment falling out of line with your risk profile.".to_string(),
//                                                                 recommended_investment_strategy: InvestmentStrategyDto::Model(
//                                                                     VersionedPortfolioDto {
//                                                                         id: ModelPortfolioIdDto {
//                                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                                             sri: false,
//                                                                             risk_profile: RiskProfileDto::Moderate,
//                                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                                         },
//                                                                         effective_date: MonthYearDto::Aug24,
//                                                                     }
//                                                                 ),
//                                                             }),
//                                                             linked_objectives: vec!["7f88927c-6da3-429e-aa7a-8600d94399e6".to_string()], 
//                                                             recommendation_actions: None
//                                                         }
//                                                     )
//                                                 }
//                                             }
//                                         )
//                                     }
//                                 ),
//                                 ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(
//                                     ExistingSingleOwnedProductDto {
//                                         id: "0a8b59f0-603d-4d54-b4c5-cc9d008c5a48".to_string(),
//                                         platform_or_account_number: None,
//                                         account_or_reference_number: AccountOrReferenceNumberTypeDto::Other("578970244765".to_string()),
//                                         account_type: AccountTypeDto::SelfInvestedPersonalPension(
//                                             SelfInvestedPersonalPensionDto {
//                                                 provider: ProviderDto::new(ProvidersDto::JamesHay),
//                                                 optional_description: None,
//                                                 current_investment_strategy: InvestmentStrategyDto::Model(
//                                                     VersionedPortfolioDto {
//                                                         id: ModelPortfolioIdDto {
//                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                             sri: false,
//                                                             risk_profile: RiskProfileDto::Moderate,
//                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                         },
//                                                         effective_date: MonthYearDto::Aug24,
//                                                     }
//                                                 ),
//                                                 current_value: ValuationDto { value: 753799.41, date_of_valuation: "11/02/2025".to_string() },
//                                                 linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto::Other("578970244765".to_string()),
//                                                 charges: ProductChargesDto {
//                                                     ongoing_advice_charge: 0.11,
//                                                     platform_charge: 0.06,
//                                                     ongoing_fund_charge: Some(0.13),
//                                                     other_charges: None
//                                                 },
//                                                 recommendations: ExistingProductRecommendationsDto {
//                                                     product_retention: ProductRetentionDto::Retain(
//                                                         RetainDto { 
//                                                             rationale: "A SIPP is a personal pension that has wider powers to invest in alternative assets, such as commercial land and property, company shares, unit trusts and similar collective investments.\nAll savings made into your pension receive tax relief. Additionally, when drawing your benefits you can take a tax-free lump sum of 25% of the pension’s total value. The rest of the value can be used to provide you with an income, taxable at your marginal rate of income tax. Any funds held within a pension are considered to be held outside of your estate for Inheritance Tax (IHT) purposes. For more information on a SIPP, please see the appendices.".to_string(), 
//                                                             recommended_product_charges: ProductChargesDto {
//                                                                 ongoing_advice_charge: 0.72,
//                                                                 platform_charge: 0.31,
//                                                                 ongoing_fund_charge: None,
//                                                                 other_charges: None
//                                                             }, 
//                                                             recommended_investment_strategy: RealignOrRebalanceDto::Rebalance(RebalanceDto {
//                                                                 rationale: "The existing fund and asset allocation selection still remains suitable and as such I recommend rebalancing the portfolio to avoid portfolio drift which can result in your investment falling out of line with your risk profile.".to_string(),
//                                                                 recommended_investment_strategy: InvestmentStrategyDto::Model(
//                                                                     VersionedPortfolioDto {
//                                                                         id: ModelPortfolioIdDto {
//                                                                             provider: InvestmentStrategyProviderDto::Transact,
//                                                                             service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                                             sri: false,
//                                                                             risk_profile: RiskProfileDto::Moderate,
//                                                                             product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                                         },
//                                                                         effective_date: MonthYearDto::Aug24,
//                                                                     }
//                                                                 ),
//                                                             }),
//                                                             linked_objectives: vec!["7f88927c-6da3-429e-aa7a-8600d94399e6".to_string()], 
//                                                             recommendation_actions: None
//                                                         }
//                                                     )
//                                                 }
//                                             }
//                                         )
//                                     }
//                                 ),
//                             ]))
//                         }),
//                         client_2: Some(
//                             AdviceAreasAndProductsDto {
//                                 advice_areas: None,
//                                 products: Some(
//                                     ProductsDto::new(vec![
//                                         ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(
//                                             ExistingSingleOwnedProductDto {
//                                                 id: "576ac603-fcf4-495c-bfac-1551e2ecf20a".to_string(),
//                                                 platform_or_account_number: Some(PlatformAccountNumberTypeDto::Transact("897-778-195".to_string())),
//                                                 account_or_reference_number: AccountOrReferenceNumberTypeDto::Transact("IH00754896".to_string()),
//                                                 account_type: AccountTypeDto::IsaStocksAndShares(
//                                                     IsaStocksAndSharesDto {
//                                                         provider: ProviderDto::new(ProvidersDto::Transact),
//                                                         optional_description: Some("New ISA".to_string()),
//                                                         current_investment_strategy: InvestmentStrategyDto::Model(
//                                                             VersionedPortfolioDto {
//                                                                 id: ModelPortfolioIdDto {
//                                                                     provider: InvestmentStrategyProviderDto::Transact,
//                                                                     service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                                     sri: false,
//                                                                     risk_profile: RiskProfileDto::Moderate,
//                                                                     product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                                 },
//                                                                 effective_date: MonthYearDto::Aug24,
//                                                             }
//                                                         ),
//                                                         current_value: ValuationDto { value: 46483.78, date_of_valuation: "11/02/2025".to_string() },
//                                                         linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto::Transact("IH00754896".to_string()),
//                                                         charges: ProductChargesDto {
//                                                             ongoing_advice_charge: 0.72,
//                                                             platform_charge: 0.26,
//                                                             ongoing_fund_charge: Some(0.45),
//                                                             other_charges: Some(OtherChargeDto {
//                                                                 ongoing_charges: Some(vec![
//                                                                     OngoingChargeDto {
//                                                                         charge_description: "Administration Wrapper Charge".to_string(),
//                                                                         charge_value: 12.00,
//                                                                         frequency: "Annually".to_string(),
//                                                                     }
//                                                                 ]),
//                                                                 incidental_charges: None
//                                                             })
//                                                         },
//                                                         recommendations: ExistingProductRecommendationsDto {
//                                                             product_retention: ProductRetentionDto::Retain(
//                                                                 RetainDto { 
//                                                                     rationale: "for the same reasons as previously outlined.".to_string(), 
//                                                                     recommended_product_charges: ProductChargesDto {
//                                                                         ongoing_advice_charge: 0.72,
//                                                                         platform_charge: 0.26,
//                                                                         ongoing_fund_charge: None,
//                                                                         other_charges: Some(OtherChargeDto {
//                                                                             ongoing_charges: Some(vec![
//                                                                                 OngoingChargeDto {
//                                                                                     charge_description: "Administration Wrapper Charge".to_string(),
//                                                                                     charge_value: 12.00,
//                                                                                     frequency: "Annually".to_string(),
//                                                                                 }
//                                                                             ]),
//                                                                             incidental_charges: None
//                                                                         })
//                                                                     }, 
//                                                                     recommended_investment_strategy: RealignOrRebalanceDto::Rebalance(RebalanceDto {
//                                                                         rationale: "The existing fund and asset allocation selection still remains suitable and as such I recommend rebalancing the portfolio to avoid portfolio drift which can result in your investment falling out of line with your risk profile.".to_string(),
//                                                                         recommended_investment_strategy: InvestmentStrategyDto::Model(
//                                                                             VersionedPortfolioDto {
//                                                                                 id: ModelPortfolioIdDto {
//                                                                                     provider: InvestmentStrategyProviderDto::Transact,
//                                                                                     service_proposition: InvestmentStrategyServicePropositionDto::Prime,
//                                                                                     sri: false,
//                                                                                     risk_profile: RiskProfileDto::Moderate,
//                                                                                     product_type: InvestmentStrategyProductTypeDto::Standard,  
//                                                                                 },
//                                                                                 effective_date: MonthYearDto::Aug24,
//                                                                             }
//                                                                         ),
//                                                                     }),
//                                                                     linked_objectives: vec!["7f88927c-6da3-429e-aa7a-8600d94399e6".to_string()], 
//                                                                     recommendation_actions: Some(vec![
//                                                                         RecommendedActionDto::SingleContribution(SingleContributionDto {
//                                                                             value: 20000.00,
//                                                                             executive_summary_description: "Receive a single contribution of £20,000.00 from your savings".to_string(),
//                                                                             rationale: Some("Cos this is better for you".to_string()),
//                                                                             date_of_action: None,
//                                                                             tax_year_of_action: None,
//                                                                         })
//                                                                     ])
//                                                                 }
//                                                             )
//                                                         }
//                                                     }
//                                                 )
//                                             }
//                                         ),
//                                     ])
//                                 )
//                             }
//                         ),
//                         joint: Some(AdviceAreasAndProductsDto {
//                             advice_areas: Some(AdviceAreasDto::new(vec![
//                                 AdviceAreaDto::Iht(IhtAdvice {
//                                     advice: "We have spoken about Inheritance Tax at length in previous meetings, and although you do not wish for advice currently regarding your IHT liability at this time, I thought I would detail your current position below:\nProperty  £4,380,000\nCash savings  £80,000\nInvestments	£383,293\nTotal		£4,843,293\nYour business will likely qualify for Business Relief (IHT exempt), so I have not included it in your estate.\nYou are both entitled to a Nil Rate Band (£325,000 each) and therefore the chargeable estate is circa £4,193,293. The estimated IHT liability on second death is £1,677,317. You have a Whole of Life policy in place with Zurich that has a sum assured of circa £775,094 which covers approximately half of the liability with the remaining having to be funded from liquid assets and possibly sale of properties.".to_string()
//                                 }),
//                                 AdviceAreaDto::Will(WillAdvice {
//                                     advice: "You've got one".to_string()
//                                 }),
//                                 AdviceAreaDto::Poa(PoaAdvice {
//                                     advice: "You're getting one".to_string()
//                                 }),
//                                 AdviceAreaDto::EmergencyFund(EmergencyFundAdvice {
//                                     advice: "You've got enough".to_string()
//                                 }),
//                             ])),
//                             products: None
//                         })   
//                     }
//                 }
//             }
//         )
//     }
// }




//             MonthYearDto::Aug24(InvestmentPortfolioDto {
                                                //                 risk_level: RiskProfileDto::Moderate,
                                                //                 fund_holdings: vec![
                                                //                     FundHoldingDto { 
                                                //                         fund_name: "Aberdeen Angus".to_string(),  
                                                //                         isin: None,
                                                //                         sedol: None,
                                                //                         value: None,
                                                //                         percentage_of_portfolio: Some(0.25),
                                                //                         fund_charge: 0.45
                                                //                     },
                                                //                     FundHoldingDto { 
                                                //                         fund_name: "Stewart Investors".to_string(),  
                                                //                         isin: None,
                                                //                         sedol: None,
                                                //                         value: None,
                                                //                         percentage_of_portfolio: Some(0.25),
                                                //                         fund_charge: 0.56
                                                //                     },
                                                //                     FundHoldingDto { 
                                                //                         fund_name: "Veritas Asian".to_string(),  
                                                //                         isin: None,
                                                //                         sedol: None,
                                                //                         value: None,
                                                //                         percentage_of_portfolio: Some(0.25),
                                                //                         fund_charge: 0.76
                                                //                     },
                                                //                     FundHoldingDto { 
                                                //                         fund_name: "Man GLG Japan Core Alpha".to_string(),  
                                                //                         isin: None,
                                                //                         sedol: None,
                                                //                         value: None,
                                                //                         percentage_of_portfolio: Some(0.25),
                                                //                         fund_charge: 0.35
                                                //                     },
                                                //                 ],
                                                //                 fund_charges: None
                                                //             })
                                                //         )
                                                //     )
                                                // ),