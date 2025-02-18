use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::risk_assessment_dto::RiskProfileDto;
use super::investment_holdings::{
    FundHoldingDto,
    InvestmentStrategyDto
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductsDto(Vec<ExistingNewJointSingleProductDto>);

impl ProductsDto {
    pub fn value(&self) -> &Vec<ExistingNewJointSingleProductDto>{
        &self.0
    }

    pub fn new(products: Vec<ExistingNewJointSingleProductDto>) -> Self {
        Self(products)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ExistingNewJointSingleProductDto {
    ExistingJointlyOwnedProduct(ExistingJointlyOwnedProductDto),
    ExistingSingleOwnedProduct(ExistingSingleOwnedProductDto),
    NewSingleOwnedProduct(NewSingleOwnedProductDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExistingJointlyOwnedProductDto {
    pub id: String, 
    pub platform_or_account_number: Option<PlatformAccountNumberTypeDto>,
    pub account_or_reference_number: AccountOrReferenceNumberTypeDto,
    pub account_type: CanBeJointlyOwnedAccountTypeDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExistingSingleOwnedProductDto {
    pub id: String,
    pub platform_or_account_number: Option<PlatformAccountNumberTypeDto>,
    pub account_or_reference_number: AccountOrReferenceNumberTypeDto,
    pub account_type: AccountTypeDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewSingleOwnedProductDto {
    pub id: String,
    pub recommendations: NewProductRecommendationsDto,
    pub platform_or_account_number: Option<PlatformAccountNumberTypeDto>,
    pub account_or_reference_number: Option<AccountOrReferenceNumberTypeDto>,
    pub account_type: AccountTypeDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CanBeJointlyOwnedAccountTypeDto {
    GeneralInvestmentAccount(GeneralInvestmentAccountDto),
    OnshoreInvestmentBond(OnshoreInvestmentBondDto),
    OffshoreInvestmentBond(OffshoreInvestmentBondDto),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AccountTypeDto {
    IsaStocksAndShares(IsaStocksAndSharesDto),
    SelfInvestedPersonalPension(SelfInvestedPersonalPensionDto),
    PersonalPension(PersonalPensionDto),
    JuniorIsaStocksAndShares(JuniorIsaStocksAndSharesDto),
    CashIsa(CashIsaDto),
    GeneralInvestmentAccount(GeneralInvestmentAccountDto),
    OnshoreInvestmentBond(OnshoreInvestmentBondDto),
    OffshoreInvestmentBond(OffshoreInvestmentBondDto),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsaStocksAndSharesDto {
    pub provider: ProviderDto,
    pub optional_description: Option<String>,
    pub current_investment_strategy: InvestmentStrategyDto,
    pub current_value: ValuationDto,
    pub linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto,
    pub charges: ProductChargesDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeneralInvestmentAccountDto {
    pub ownership: OwnershipDto,
    pub provider: ProviderDto,
    pub optional_description: Option<String>,
    pub current_investment_strategy: InvestmentStrategyDto,
    pub current_value: ValuationDto,
    pub linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto,
    pub charges: ProductChargesDto,
    pub current_tax_position: CapitalGainsPositionDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnshoreInvestmentBondDto {
    pub ownership: OwnershipDto,
    pub provider: ProviderDto,
    pub optional_description: Option<String>,
    pub current_investment_strategy: InvestmentStrategyDto,
    pub current_value: ValuationDto,
    pub linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto,
    pub charges: ProductChargesDto,
    pub current_tax_position: ChargeableGainsPositionDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OffshoreInvestmentBondDto {
    pub ownership: OwnershipDto,
    pub provider: ProviderDto,
    pub platform_account_number: PlatformAccountNumberTypeDto,
    pub account_or_reference_number: AccountOrReferenceNumberTypeDto,
    pub optional_description: Option<String>,
    pub current_investment_strategy: InvestmentStrategyDto,
    pub current_value: ValuationDto,
    pub linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto,
    pub charges: ProductChargesDto,
    pub current_tax_position: ChargeableGainsPositionDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SelfInvestedPersonalPensionDto {
    pub provider: ProviderDto,
    pub optional_description: Option<String>,
    pub current_investment_strategy: InvestmentStrategyDto,
    pub current_value: ValuationDto,
    pub linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto,
    pub charges: ProductChargesDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPensionDto {
    pub provider: ProviderDto,
    pub optional_description: Option<String>,
    pub current_investment_strategy: InvestmentStrategyDto,
    pub current_value: ValuationDto,
    pub linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto,
    pub charges: ProductChargesDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JuniorIsaStocksAndSharesDto {
    pub provider: ProviderDto,
    pub optional_description: Option<String>,
    pub current_investment_strategy: InvestmentStrategyDto,
    pub current_value: ValuationDto,
    pub linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberTypeDto,
    pub charges: ProductChargesDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CashIsaDto {
    pub provider: ProviderDto,
    pub account_number: String,
    pub sort_code: String,
    pub optional_description: Option<String>,
    pub current_value: ValuationDto,
    pub recommendations: ExistingProductRecommendationsDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipDto {
    pub client_first_name: String,
    pub client_last_name: String,
    pub percentage_owned: f32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDto(ProvidersDto);

impl ProviderDto {
    /// Returns the value of the ProviderDto.
    pub fn value(&self) -> &ProvidersDto {
        &self.0
    }

    pub fn new(providers: ProvidersDto) -> Self {
        Self(providers)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ProvidersDto {
    Abrdn,
    Transact,
    Utmost,
    ReAssure,
    Quilter,
    Fidelity,
    JamesHay
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValuationDto {
    pub value: f64,
    pub date_of_valuation: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductChargesDto {
    pub ongoing_advice_charge: f32,
    pub platform_charge: f32,
    pub ongoing_fund_charge: Option<f32>,
    pub other_charges: Option<OtherChargeDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentProductTaxPositionDto {
    pub product_tax_position: ProductTaxPositionDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExistingProductRecommendationsDto {
    //pub recommended_product_charges: ProductChargesDto,
    pub product_retention: ProductRetentionDto,
    //pub recommended_investment_strategy: InvestmentStrategyDto,
    //pub linked_objectives: Vec<Uuid>,
    //pub recommendation_actions: Vec<RecommendedActionDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewProductRecommendationsDto {
    pub rationale: String,
    pub recommended_product_charges: ProductChargesDto,
    pub recommended_investment_strategy: InvestmentStrategyDto,
    pub linked_objectives: Vec<Uuid>,
    pub recommendation_actions: Vec<RecommendedActionDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherChargeDto {
    pub ongoing_charges: Option<Vec<OngoingChargeDto>>,
    pub incidental_charges: Option<Vec<IncidentalChargeDto>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedInvestmentAndRiskStrategyDto {
    pub recommended_investment_strategy: InvestmentStrategyDto,
    pub risk_level: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum CapitalGainsPositionDto {
    CapitalGainsTaxAvoidLiability(CapitalGainsTaxAvoidLiabilityDto),
    CapitalGainsTaxNoLiability(CapitalGainsTaxNoLiabilityDto),
    CapitalGainsTaxIncurLiability(CapitalGainsTaxIncurLiabilityDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChargeableGainsPositionDto {
    ChargeableGainsTaxAvoidLiability(ChargeableGainsTaxAvoidLiabilityDto),
    ChargeableGainsTaxNoLiability(ChargeableGainsTaxNoLiabilityDto),
    ChargeableGainsTaxIncurLiability(ChargeableGainsTaxIncurLiabilityDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum ProductRetentionDto {
    Retain(RetainDto),
    Replace(ReplaceDto), 
    FullyEncash(FullyEncashDto),
    //PartialTransfer(PartialTransferDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RecommendedActionDto {
    SingleWithdrawal(SingleWithdrawalDto),
    SingleContribution(SingleContributionDto),
    RegularContribution(RegularContributionDto),
    RegularWithdrawal(RegularWithdrawalDto),
    Transfer(TransferDto),
    StopWithdrawal(StopWithdrawalDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum ProductTaxPositionDto {
    CapitalGainsTaxPositionDto(CapitalGainsPositionDto),
    ChargeableGainsPositionDto(ChargeableGainsPositionDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OngoingChargeDto {
    pub charge_description: String,
    pub charge_value: f32,
    pub frequency: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncidentalChargeDto {
    pub charge_description: String,
    pub charge_value: f32,
    pub frequency: String,
    pub trigger_event: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxAvoidLiabilityDto {
    pub unrealised_gains: f32,
    pub capital_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxNoLiabilityDto {
    pub unrealised_gains: f32,
    pub capital_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxIncurLiabilityDto {
    pub unrealised_gains: f32,
    pub capital_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxAvoidLiabilityDto {
    pub unrealised_gains: f32,
    pub chargeable_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxNoLiabilityDto {
    pub unrealised_gains: f32,
    pub chargeable_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxIncurLiabilityDto {
    pub unrealised_gains: f32,
    pub chargeable_gains_tax_discussion: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RetainDto {
    pub rationale: String,
    pub recommended_product_charges: ProductChargesDto,
    pub recommended_investment_strategy: RealignOrRebalanceDto,
    pub linked_objectives: Vec<String>,
    pub recommendation_actions: Option<Vec<RecommendedActionDto>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RealignOrRebalanceDto {
    Realign(RealignDto),
    Rebalance(RebalanceDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RealignDto {
    pub rationale: String,
    pub recommended_investment_strategy: InvestmentStrategyDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RebalanceDto {
    pub rationale: String,
    pub recommended_investment_strategy: InvestmentStrategyDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReplaceDto {
    FullyReplace(FullyReplaceDto),
    PartiallyReplace(PartiallyReplaceDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullyReplaceDto {
    pub rationale: String,
    pub replacement_product_information: ReplacementProductInformationDto,
    pub replace_to_details: FullyReplaceDetailDto,
    pub linked_objectives: Vec<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullyReplaceDetailDto {
    pub method_of_transfer: Vec<MethodOfTransferDto>,
    pub transfer_to_account_or_reference_number: AccountOrReferenceNumberTypeDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MethodOfTransferDto {
    InSpecieMethod(InSpecieMethodDto),
    CashMethod(CashMethodDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InSpecieMethodDto {
    pub value: f64,
    pub funds_to_inspecie_transfer: Vec<FundHoldingDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CashMethodDto {
    pub value: f64
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartiallyReplaceDto {
    pub rationale: String,
    pub replacement_product_information: ReplacementProductInformationDto,
    pub partially_replace_to_details: PartiallyReplaceDetailDto,
    pub linked_objectives: Vec<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartiallyReplaceDetailDto {
    pub method_of_transfer: Vec<MethodOfTransferDto>,
    pub transfer_to_account_or_reference_number: AccountOrReferenceNumberTypeDto,
    pub amount_to_be_left_in_existing_product: f64,
    pub reason_for_leaving_in_existing_product: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartialTransferDto {
    pub rationale: String,
    pub value_to_transfer: f64,
    pub replacement_product_information: ReplacementProductInformationDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullyEncashDto {
    pub rationale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingleWithdrawalDto {
    pub value: f64,
    pub executive_summary_description: String,
    pub rationale: String,
    pub date_of_action: Option<String>,
    pub tax_year_of_action: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingleContributionDto {
    pub value: f64,
    pub executive_summary_description: String,
    pub rationale: Option<String>,
    pub date_of_action: Option<String>,
    pub tax_year_of_action: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegularContributionDto {
    pub value: f64,
    pub executive_summary_description: String,
    pub rationale: String,
    pub frequency: String,
    pub start_date_of_action: String,
    pub tax_year_of_action: Option<String>,
    pub end_date_of_action: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegularWithdrawalDto {
    pub value: f64,
    pub executive_summary_description: String,
    pub rationale: String,
    pub frequency: String,
    pub start_date_of_action: String,
    pub tax_year_of_action: Option<String>,
    pub end_date_of_action: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferDto {
    pub id: String,
    pub value: f64,
    pub executive_summary_description_receiving_product: Option<String>,
    pub executive_summary_description_transferring_product: Option<String>,
    pub rationale: String,
    pub date_of_action: Option<String>,
    pub tax_year_of_action: Option<String>,
    pub transfer_details: TransferDetailDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopWithdrawalDto {
    pub value: f64,
    pub executive_summary_description: String,
    pub rationale: String,
    pub start_date_of_action: Option<String>,
    pub tax_year_of_action: Option<String>,
    pub end_date_of_action: Option<String>,
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct InvestmentStrategyDto {
//     pub risk_level: RiskProfileDto,
//     pub fund_allocations: BespokeOrFirmInvestmentStrategyDto
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferDetailDto {
    pub transfer_to_account_or_reference_number: AccountOrReferenceNumberTypeDto,
    pub transfer_from_account_or_reference_number: KnownOrUnknownAccountDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum KnownOrUnknownAccountDto {
    Known(AccountOrReferenceNumberTypeDto),
    Unknown { description: String, account_type: String }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReplacementProductInformationDto {
    PensionReplacementProductInformation(PensionReplacementProductInformationDto),
    InvestmentReplacementProductInformation(InvestmentReplacementProductInformationDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum BespokeOrFirmInvestmentStrategyDto {
    BespokeInvestmentStrategy(BespokeInvestmentStrategyDto),
    FirmInvestmentStrategy(PresentFirmInvestmentStrategyDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BespokeInvestmentStrategyDto {
    pub description: String,
    pub fund_allocation: Option<FundHoldingDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PresentFirmInvestmentStrategyDto {
    PrimeModerate(Vec<FundHoldingDto>)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PensionReplacementProductInformationDto {
    pub start_date: String,
    pub total_contributions: f64,
    pub current_transfer_value: f64,
    pub no_of_funds_available: i32,
    pub max_number_of_funds_invested_at_one_time: Option<i32>,
    pub retirement_date_age: i32,
    pub is_waiver_of_premium_insurance_available: bool,
    pub death_benefits_description: String,
    pub is_life_cover_available: Option<f64>,
    pub loyalty_bonus: Option<f32>,
    pub fund_bonus_enhanced_allocation: Option<f32>,
    pub tax_free_cash_entitlement: f32,
    pub is_flexi_access_available: bool,
    pub is_full_ufpls_available: bool,
    pub is_partial_ufpls_available: bool,
    pub is_transfers_contributions_allowed_in: bool,
    pub is_block_or_bulk_transfer_received: bool,
    pub is_enhanced_protection_available: bool,
    pub is_earmarking_order: bool,
    pub is_charge_guarantee_and_guarantee_amount: bool,
    pub is_existing_pension_sharing_order: bool,
    pub is_guaranteed_minimum_fund: bool,
    pub is_guaranteed_minimum_annuity: bool,
    pub is_guaranteed_minimum_pension_or_reference_scheme_test: bool,
    pub is_guaranteed_annuity_rates: bool,
    pub other_features: Vec<(String, String)>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentReplacementProductInformationDto {
    pub start_date: String,
    pub total_contributions: f64,
    pub current_transfer_value: f64,
    pub no_of_funds_available: i32,
    pub max_number_of_funds_invested_at_one_time: Option<i32>,
    pub loyalty_bonus: Option<f32>,
    pub fund_bonus_enhanced_allocation: Option<f32>,
    pub is_charge_guarantee_and_guarantee_amount: bool,
    pub is_guaranteed_return_applicable: bool,
    pub other_features: Vec<(String, String)>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PlatformAccountNumberTypeDto {
    Abrdn(String),
    Transact(String),
    Other(String)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum AccountOrReferenceNumberTypeDto {
    Abrdn(String),
    AbrdnSipp(String),
    Transact(String),
    Other(String),
    NewAccount(String)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PlatformOrAccountReferenceNumberTypeDto {
    PlatformAccountNumberType(PlatformAccountNumberTypeDto),
    AccountOrReferenceNumberType(AccountOrReferenceNumberTypeDto)
}
