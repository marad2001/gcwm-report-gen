use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::risk_assessment_dto::RiskProfileDto;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ExistingNewJointSingleProductDto {
    ExistingJointlyOwnedProduct(ExistingJointlyOwnedProductDto),
    ExistingSingleOwnedPRoduct(ExistingSingleOwnedProductDto),
    NewSingleOwnedProduct(NewSingleOwnedProductDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExistingJointlyOwnedProductDto {
    ownership: OwnershipDto,
    provider: ProviderDto,
    platform_account_number: PlatformAccountNumberTypeDto,
    account_or_reference_number: AccountOrReferenceNumberTypeDto,
    optional_description: String,
    tax_wrapper_type: TaxWrapperTypeDto,
    current_investment_strategy: CurrentInvestmentStrategyDto,
    current_value: ValuationDto,
    linked_cash_or_fee_payment_wrapper: PlatformOrAccountReferenceNumberTypeDto,
    charges: ProductChargesDto,
    current_tax_position: CurrentProductTaxPositionDto,
    recommendations: ExistingProductRecommendationsDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExistingSingleOwnedProductDto {
    provider: ProviderDto,
    platform_account_number: PlatformAccountNumberTypeDto,
    account_or_reference_number: AccountOrReferenceNumberTypeDto,
    optional_description: String,
    tax_wrapper_type: TaxWrapperTypeDto,
    current_investment_strategy: CurrentInvestmentStrategyDto,
    current_value: ValuationDto,
    linked_cash_or_fee_payment_wrapper: PlatformOrAccountReferenceNumberTypeDto,
    charges: ProductChargesDto,
    current_tax_position: CurrentProductTaxPositionDto,
    recommendations: ExistingProductRecommendationsDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSingleOwnedProductDto {
    provider: ProviderDto,
    platform_account_number: PlatformAccountNumberTypeDto,
    account_or_reference_number: AccountOrReferenceNumberTypeDto,
    optional_description: String,
    tax_wrapper_type: TaxWrapperTypeDto,
    linked_cash_or_fee_payment_wrapper: PlatformOrAccountReferenceNumberTypeDto,
    charges: ProductChargesDto,
    recommendations: ExistingProductRecommendationsDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipDto {
    client_first_name: String,
    client_last_name: String,
    percentage_owned: f32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDto(ProvidersDto);

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ProvidersDto {
    Abrdn,
    Transact,
    Utmost,
    ReAssure,
    Quilter
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum TaxWrapperTypeDto {
    IsaStocksAndShares,
    GeneralInvestmentAccount,
    OnshoreInvestmentBond,
    OffshoreInvestmentBond,
    SelfInvestedPersonalPension,
    PersonalPension,
    JuniorIsaStocksAndShares
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CurrentInvestmentStrategyDto {
    GCWMInvestmentStrategy(GCWMInvestmentStrategiesDto),
    OtherInvestmentStrategy(OtherInvestmentStrategyDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GCWMInvestmentStrategiesDto {
    PrimeModerate(CurrentInvestmentStrategyMonthYearDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum CurrentInvestmentStrategyMonthYearDto {
    Aug24
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherInvestmentStrategyDto {
    description: f32,
    fund_allocation: Option<FundHoldingDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundHoldingDto {
    fund_name: String,
    isin: Option<String>,
    sedol: Option<String>,
    value: Option<f32>,
    percentage_of_portfolio: Option<f32>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValuationDto {
    value: f32,
    date_of_valuation: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductChargesDto {
    ongoing_advice_charge: f32,
    platform_charge: f32,
    ongoing_fund_charge: f32,
    other_charges: OtherChargeDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentProductTaxPositionDto {
    product_tax_position: ProductTaxPositionDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExistingProductRecommendationsDto {
    recommended_product_charges: ProductChargesDto,
    product_retention: ProductRetentionDto,
    recommended_investment_strategy: RecommendedInvestmentAndRiskStrategyDto,
    linked_objectives: Vec<Uuid>,
    recommendation_actions: Vec<RecommendedActionDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewProductRecommendationsDto {
    recommended_product_charges: ProductChargesDto,
    recommended_investment_strategy: RecommendedInvestmentAndRiskStrategyDto,
    linked_objectives: Vec<Uuid>,
    recommendation_actions: Vec<RecommendedActionDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherChargeDto {
    ongoing_charges: Vec<OngoingChargeDto>,
    incidental_charges: Vec<IncidentalChargeDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedInvestmentAndRiskStrategyDto {
    recommended_investment_strategy: InvestableInvestmentStrategyDto,
    risk_level: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum CapitalGainsPositionDto {
    CapitalGainsTaxAvoidLiability,
    CapitalGainsTaxNoLiability,
    CapitalGainsTaxIncurLiability
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChargeableGainsPositionDto {
    ChargeableGainsTaxAvoidLiability(CapitalGainsTaxAvoidLiabilityDto),
    ChargeableGainsTaxNoLiability(CapitalGainsTaxNoLiabilityDto),
    ChargeableGainsTaxIncurLiability(CapitalGainsTaxIncurLiabilityDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ProductRetentionDto {
    Retain,
    Replace, 
    FullyEncash,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RecommendedActionDto {
    SingleWithdrawal,
    SingleContribution,
    RegularContribution,
    RegularWithdrawal,
    Transfer,
    StopWithdrawal
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ProductTaxPositionDto {
    CapitalGainsTaxPositionDto,
    ChargeableGainsPositionDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OngoingChargeDto {
    charge_description: String,
    charge_value: f32,
    frequency: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncidentalChargeDto {
    charge_description: String,
    charge_value: f32,
    frequency: String,
    trigger_event: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxAvoidLiabilityDto {
    unrealised_gains: f32,
    capital_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxNoLiabilityDto {
    unrealised_gains: f32,
    capital_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxIncurLiabilityDto {
    unrealised_gains: f32,
    capital_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxAvoidLiabilityDto {
    unrealised_gains: f32,
    chargeable_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxNoLiabilityDto {
    unrealised_gains: f32,
    chargeable_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxIncurLiabilityDto {
    unrealised_gains: f32,
    chargeable_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RetainDto {
    rationale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceDto {
    rationale: String,
    replacement_product_information: ReplacementProductInfromation
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullyEncashDto {
    rationale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingleWithdrawalDto {
    value: f32,
    executive_summary_description: String,
    rationale: String,
    date_of_action: Option<String>,
    tax_year_of_action: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingleContributionDto {
    value: f32,
    executive_summary_description: String,
    rationale: String,
    date_of_action: Option<String>,
    tax_year_of_action: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegularContributionDto {
    value: f32,
    executive_summary_description: String,
    rationale: String,
    start_date_of_action: String,
    tax_year_of_action: Option<String>,
    end_date_of_action: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegularWithdrawalDto {
    value: f32,
    executive_summary_description: String,
    rationale: String,
    start_date_of_action: String,
    tax_year_of_action: Option<String>,
    end_date_of_action: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferDto {
    value: f32,
    executive_summary_description: String,
    rationale: String,
    date_of_action: Option<String>,
    tax_year_of_action: Option<String>,
    transfer_to_details: Vec<TransferDetailDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopWithdrawalDto {
    value: f32,
    executive_summary_description: String,
    rationale: String,
    start_date_of_action: Option<String>,
    tax_year_of_action: Option<String>,
    end_date_of_action: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvestableInvestmentStrategyDto {
    risk_level: RiskProfileDto,
    fund_allocations: BespokeOrFirmInvestmentStrategyDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferDetailDto {
    value: f32,
    transfer_to: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReplacementProductInfromation {
    PensionReplacementProductInformation,
    InvestmentReplacementProductInformation
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BespokeOrFirmInvestmentStrategyDto {
    BespokeInvestmentStrategyDto(BespokeInvestmentStrategyDto),
    FirmInvestmentStrategyDto(PresentFirmInvestmentStrategyDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BespokeInvestmentStrategyDto {
    description: f32,
    fund_allocation: Option<FundHoldingDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PresentFirmInvestmentStrategyDto {
    PrimeModerate(Vec<FundHoldingDto>)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PensionReplacementProductInformationDto {
    start_date: String,
    total_contributions: f32,
    current_transfer_value: f32,
    no_of_funds_available: f32,
    max_number_of_funds_invested_at_one_time: Option<f32>,
    retirement_date_age: f32,
    is_waiver_of_premium_insurance_available: bool,
    death_benefits_description: String,
    is_life_cover_available: Option<f32>,
    loyalty_bonus: Option<f32>,
    fund_bonus_enhanced_allocation: Option<f32>,
    tax_free_cash_entitlement: f32,
    is_flexi_access_available: bool,
    is_full_ufpls_available: bool,
    is_partial_ufpls_available: bool,
    is_transfers_contributions_allowed_in: bool,
    is_block_or_bulk_transfer_received: bool,
    is_enhanced_protection_available: bool,
    is_earmarking_order: bool,
    is_charge_guarantee_and_guarantee_amount: bool,
    is_existing_pension_sharing_order: bool,
    is_guaranteed_minimum_fund: bool,
    is_guaranteed_minimum_annuity: bool,
    is_guaranteed_minmum_pension_or_reference_scheme_test: bool,
    is_guaranteed_annuity_rates: bool,
    other_features: Vec<(String, String)>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentReplacementProductInformationDto {
    start_date: String,
    total_contributions: f32,
    current_transfer_value: f32,
    no_of_funds_available: f32,
    max_number_of_funds_invested_at_one_time: Option<f32>,
    loyalty_bonus: Option<f32>,
    fund_bonus_enhanced_allocation: Option<f32>,
    is_charge_guarantee_and_guarantee_amount: bool,
    is_guaranteed_return_applicable: bool,
    other_features: Vec<(String, String)>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PlatformAccountNumberTypeDto {
    Abrdn(String),
    Transact(String),
    Other(String)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum AccountOrReferenceNumberTypeDto {
    Abrdn(String),
    Transact(String),
    Other(String)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PlatformOrAccountReferenceNumberTypeDto {
    PlatformAccountNumberTypeDto(PlatformAccountNumberTypeDto),
    AccountOrReferenceNumberTypeDto(AccountOrReferenceNumberTypeDto)
}

