use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::convert::TryFrom;
use std::fmt;

use crate::domain::constrained_types::abrdn_account_number::AbrdnAccountNumber;
use crate::domain::constrained_types::abrdn_full_account_number::AbrdnFullAccountNumber;
use crate::domain::constrained_types::abrdn_sipp_number::AbrdnSippNumber;
use crate::domain::constrained_types::transact_platform_number::TransactPlatformNumber;
use crate::domain::constrained_types::transact_reference_number::TransactReferenceNumber;
use crate::domain::constrained_types::{
    constrained_money_amount_large::ConstrainedMoneyAmountLarge,
    constrained_money_amount_medium::ConstrainedMoneyAmountMedium,
    constrained_money_amount_small::ConstrainedMoneyAmountSmall,
    constrained_string_1000::ConstrainedString1000,
    constrained_string_200::ConstrainedString200,
    frequency::Frequency,
    fund_bonus_allocation::FundBonusAllocation,
    isin::ISIN,
    name_string::NameString,
    percentage::Percentage,
    product_retirement_age::ProductRetirementAge,
    protected_tax_free_cash_entitlement::ProtectedTaxFreeCashEntitlement,
    sedol::Sedol,
};
use crate::domain::constrained_types::date::Date;
use crate::domain::constrained_types::tax_year::TaxYear;
use super::risk_assessment::RiskProfile;
use crate::driving::data_transfer_object::report_type_data_transfer_object::product::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Products(Vec<ExistingNewJointSingleProduct>);

impl Products {
    pub fn value(&self) -> &Vec<ExistingNewJointSingleProduct> {
        &self.0
    }
}

impl TryFrom<ProductsDto> for Products {
    type Error = String;

    fn try_from(dto: ProductsDto) -> Result<Self, Self::Error> {
        Ok(Products(dto.value().into_iter().map(|dto| dto.clone().try_into()).collect::<Result<_, _>>()?))
    }
}

impl Products {

    pub fn existing_products(&self) -> Vec<ExistingProduct> {
        self.0
            .iter()
            .filter_map(|product| match product {
                ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(p) => {
                    Some(ExistingProduct::JointlyOwned(p.clone()))
                }
                ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(p) => {
                    Some(ExistingProduct::SingleOwned(p.clone()))
                }
                ExistingNewJointSingleProduct::NewSingleOwnedProduct(_) => None, // Ignore new products
            })
            .collect()
    }
    
    /// Returns a vector of all new single-owned products.
    pub fn new_products(&self) -> Vec<NewProduct> {
        self.0
            .iter()
            .filter_map(|product| match product {
                ExistingNewJointSingleProduct::NewSingleOwnedProduct(p) => {
                    Some(NewProduct::SingleOwned(p.clone()))
                }
                _ => None, // Ignore existing products
            })
            .collect()
    }
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ExistingProduct {
    JointlyOwned(ExistingJointlyOwnedProduct),
    SingleOwned(ExistingSingleOwnedProduct),
}

impl ExistingProduct {
    /// Returns the provider as a string.
    pub fn provider_as_string(&self) -> String {
        match self {
            ExistingProduct::JointlyOwned(product) => product.provider.0.to_string(),
            ExistingProduct::SingleOwned(product) => product.provider.0.to_string(),
        }
    }

    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        match self {
            ExistingProduct::JointlyOwned(product) => product.tax_wrapper_type.to_string(),
            ExistingProduct::SingleOwned(product) => product.tax_wrapper_type.to_string(),
        }
    }

    /// Returns the account or reference number as a string.
    pub fn account_or_reference_number_as_string(&self) -> String {
        match self {
            ExistingProduct::JointlyOwned(product) => product.account_or_reference_number.to_string(),
            ExistingProduct::SingleOwned(product) => product.account_or_reference_number.to_string(),
        }
    }

    /// Returns a reference to the provider.
    pub fn provider(&self) -> &Provider {
        match self {
            ExistingProduct::JointlyOwned(product) => &product.provider,
            ExistingProduct::SingleOwned(product) => &product.provider,
        }
    }

    /// Returns a reference to the retention recommendation.
    pub fn product_retention(&self) -> &ProductRetention {
        match self {
            ExistingProduct::JointlyOwned(product) => &product.recommendations.product_retention,
            ExistingProduct::SingleOwned(product) => &product.recommendations.product_retention,
        }
    }
    
    
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NewProduct {
    SingleOwned(NewSingleOwnedProduct),
}

impl NewProduct {
    /// Returns the provider as a string.
    pub fn provider_as_string(&self) -> String {
        match self {
            NewProduct::SingleOwned(product) => product.provider.0.to_string(),
        }
    }

    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        match self {
            NewProduct::SingleOwned(product) => product.tax_wrapper_type.to_string(),
        }
    }

    /// Returns the account or reference number as a string.
    pub fn account_or_reference_number_as_string(&self) -> String {
        match self {
            NewProduct::SingleOwned(product) => product.account_or_reference_number.to_string(),
        }
    }

   
    /// Returns a reference to the provider.
    pub fn provider(&self) -> &Provider {
        match self {
            NewProduct::SingleOwned(product) => &product.provider,
        }
    }
 
    
}




#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ExistingNewJointSingleProduct {
    ExistingJointlyOwnedProduct(ExistingJointlyOwnedProduct),
    ExistingSingleOwnedProduct(ExistingSingleOwnedProduct),
    NewSingleOwnedProduct(NewSingleOwnedProduct),
}

impl TryFrom<ExistingNewJointSingleProductDto> for ExistingNewJointSingleProduct {
    type Error = String;

    fn try_from(dto: ExistingNewJointSingleProductDto) -> Result<Self, Self::Error> {
        match dto {
            ExistingNewJointSingleProductDto::ExistingJointlyOwnedProduct(data) => {
                Ok(ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(data.try_into()?))
            }
            ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(data) => {
                Ok(ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(data.try_into()?))
            }
            ExistingNewJointSingleProductDto::NewSingleOwnedProduct(data) => {
                Ok(ExistingNewJointSingleProduct::NewSingleOwnedProduct(data.try_into()?))
            }
        }
    }
}

impl ExistingNewJointSingleProduct {
    
    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                product.tax_wrapper_type.to_string()
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                product.tax_wrapper_type.to_string()
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                product.tax_wrapper_type.to_string()
            }
        }
    }

    /// Returns the account or reference number as a string.
    pub fn account_or_reference_number_as_string(&self) -> String {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                product.account_or_reference_number.to_string()
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                product.account_or_reference_number.to_string()
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                product.account_or_reference_number.to_string()
            }
        }
    }

    /// Returns the platform account number as a string.
    pub fn platform_account_number_as_string(&self) -> String {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                product.platform_account_number.to_string()
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                product.platform_account_number.to_string()
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                product.platform_account_number.to_string()
            }
        }
    }

    /// Returns the provider.
    pub fn provider(&self) -> &Provider {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                &product.provider
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                &product.provider
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                &product.provider
            }
        }
    }

    /// Returns the provider as a string.
    pub fn provider_as_string(&self) -> String {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                product.provider.0.to_string()  // Access inner Providers enum
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                product.provider.0.to_string()
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                product.provider.0.to_string()
            }
        }
    }



}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExistingJointlyOwnedProduct {
    ownership: Ownership,
    provider: Provider,
    platform_account_number: PlatformAccountNumberType,
    account_or_reference_number: AccountOrReferenceNumberType,
    optional_description: Option<ConstrainedString200>,
    tax_wrapper_type: TaxWrapperType,
    current_investment_strategy: CurrentInvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: PlatformOrAccountReferenceNumberType,
    charges: ProductCharges,
    current_tax_position: Option<CurrentProductTaxPosition>,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<ExistingJointlyOwnedProductDto> for ExistingJointlyOwnedProduct {
    type Error = String;

    fn try_from(dto: ExistingJointlyOwnedProductDto) -> Result<Self, Self::Error> {
        Ok(ExistingJointlyOwnedProduct {
            ownership: dto.ownership.try_into()?,
            provider: dto.provider.try_into()?,
            platform_account_number: dto.platform_account_number.try_into()?,
            account_or_reference_number: dto.account_or_reference_number.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            tax_wrapper_type: dto.tax_wrapper_type.try_into()?,
            current_investment_strategy: dto.current_investment_strategy.try_into()?,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            current_tax_position: dto.current_tax_position.map(|dto| dto.try_into()).transpose()?,
            recommendations: dto.recommendations.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExistingSingleOwnedProduct {
    provider: Provider,
    platform_account_number: PlatformAccountNumberType,
    account_or_reference_number: AccountOrReferenceNumberType,
    optional_description: Option<ConstrainedString200>,
    tax_wrapper_type: TaxWrapperType,
    current_investment_strategy: CurrentInvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: PlatformOrAccountReferenceNumberType,
    charges: ProductCharges,
    current_tax_position: Option<CurrentProductTaxPosition>,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<ExistingSingleOwnedProductDto> for ExistingSingleOwnedProduct {
    type Error = String;

    fn try_from(dto: ExistingSingleOwnedProductDto) -> Result<Self, Self::Error> {
        Ok(ExistingSingleOwnedProduct {
            provider: dto.provider.try_into()?,
            platform_account_number: dto.platform_account_number.try_into()?,
            account_or_reference_number: dto.account_or_reference_number.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            tax_wrapper_type: dto.tax_wrapper_type.try_into()?,
            current_investment_strategy: dto.current_investment_strategy.try_into()?,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            current_tax_position: dto.current_tax_position.map(|dto| dto.try_into()).transpose()?,
            recommendations: dto.recommendations.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSingleOwnedProduct {
    provider: Provider,
    platform_account_number: PlatformAccountNumberType,
    account_or_reference_number: AccountOrReferenceNumberType,
    optional_description: ConstrainedString200,
    tax_wrapper_type: TaxWrapperType,
    linked_cash_or_fee_payment_wrapper: PlatformOrAccountReferenceNumberType,
    charges: ProductCharges,
    recommendations: NewProductRecommendations,
}

impl TryFrom<NewSingleOwnedProductDto> for NewSingleOwnedProduct {
    type Error = String;

    fn try_from(dto: NewSingleOwnedProductDto) -> Result<Self, Self::Error> {
        Ok(NewSingleOwnedProduct {
            provider: dto.provider.try_into()?,
            platform_account_number: dto.platform_account_number.try_into()?,
            account_or_reference_number: dto.account_or_reference_number.try_into()?,
            optional_description: dto.optional_description.try_into()?,
            tax_wrapper_type: dto.tax_wrapper_type.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            recommendations: dto.recommendations.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ownership {
    client_first_name: NameString,
    client_last_name: NameString,
    percentage_owned: Percentage,
}

impl TryFrom<OwnershipDto> for Ownership {
    type Error = String;

    fn try_from(dto: OwnershipDto) -> Result<Self, Self::Error> {
        Ok(Ownership {
            client_first_name: dto.client_first_name.try_into()?,
            client_last_name: dto.client_last_name.try_into()?,
            percentage_owned: dto.percentage_owned.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Provider(Providers);

impl TryFrom<ProviderDto> for Provider {
    type Error = String;

    fn try_from(dto: ProviderDto) -> Result<Self, Self::Error> {

        let providers = dto.value();

        match providers {
            ProvidersDto::Abrdn => Ok(Provider(Providers::Abrdn)),
            ProvidersDto::Transact => Ok(Provider(Providers::Transact)),
            ProvidersDto::Utmost => Ok(Provider(Providers::Utmost)),
            ProvidersDto::ReAssure => Ok(Provider(Providers::ReAssure)),
            ProvidersDto::Quilter => Ok(Provider(Providers::Quilter)),
        }
    }
}

impl Provider {
    pub fn value(&self) -> &Providers {
        &self.0
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Providers {
    Abrdn,
    Transact,
    Utmost,
    ReAssure,
    Quilter,
}

impl fmt::Display for Providers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let provider_str = match self {
            Providers::Abrdn => "abrdn",  // lowercase 'a' for abrdn
            Providers::Transact => "Transact",
            Providers::Utmost => "Utmost",
            Providers::ReAssure => "ReAssure",
            Providers::Quilter => "Quilter",
        };
        write!(f, "{}", provider_str)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum TaxWrapperType {
    IsaStocksAndShares,
    GeneralInvestmentAccount,
    OnshoreInvestmentBond,
    OffshoreInvestmentBond,
    SelfInvestedPersonalPension,
    PersonalPension,
    JuniorIsaStocksAndShares
}

impl TryFrom<TaxWrapperTypeDto> for TaxWrapperType {
    type Error = String;

    fn try_from(dto: TaxWrapperTypeDto) -> Result<Self, Self::Error> {
        match dto {
            TaxWrapperTypeDto::GeneralInvestmentAccount => Ok(Self::GeneralInvestmentAccount),
            TaxWrapperTypeDto::IsaStocksAndShares => Ok(Self::IsaStocksAndShares),
            TaxWrapperTypeDto::OnshoreInvestmentBond => Ok(Self::OnshoreInvestmentBond),
            TaxWrapperTypeDto::OffshoreInvestmentBond => Ok(Self::OffshoreInvestmentBond),
            TaxWrapperTypeDto::SelfInvestedPersonalPension => Ok(Self::SelfInvestedPersonalPension),
            TaxWrapperTypeDto::PersonalPension => Ok(Self::PersonalPension),
            TaxWrapperTypeDto::JuniorIsaStocksAndShares => Ok(Self::JuniorIsaStocksAndShares)
        }
    }
}

// Implement Display for TaxWrapperType.
impl fmt::Display for TaxWrapperType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            TaxWrapperType::IsaStocksAndShares => "ISA Stocks and Shares",
            TaxWrapperType::GeneralInvestmentAccount => "General Investment Account",
            TaxWrapperType::OnshoreInvestmentBond => "Onshore Investment Bond",
            TaxWrapperType::OffshoreInvestmentBond => "Offshore Investment Bond",
            TaxWrapperType::SelfInvestedPersonalPension => "Self Invested Personal Pension",
            TaxWrapperType::PersonalPension => "Personal Pension",
            TaxWrapperType::JuniorIsaStocksAndShares => "Junior ISA Stocks and Shares",
        };
        write!(f, "{}", type_str)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CurrentInvestmentStrategy {
    GCWMInvestmentStrategy(GCWMInvestmentStrategies),
    OtherInvestmentStrategy(OtherInvestmentStrategy)
}

impl TryFrom<CurrentInvestmentStrategyDto> for CurrentInvestmentStrategy {
    type Error = String;

    fn try_from(dto: CurrentInvestmentStrategyDto) -> Result<Self, Self::Error> {
        match dto {
            CurrentInvestmentStrategyDto::GCWMInvestmentStrategy(gcwm_investment_strategy_dto) => {
                match gcwm_investment_strategy_dto {
                    GCWMInvestmentStrategiesDto::PrimeModerate(current_investment_strategy_month_year) => {
                        match current_investment_strategy_month_year {
                            CurrentInvestmentStrategyMonthYearDto::Aug24 => Ok(
                                CurrentInvestmentStrategy::GCWMInvestmentStrategy(
                                    GCWMInvestmentStrategies::PrimeModerate(
                                        CurrentInvestmentStrategyMonthYear::Aug24
                                    )
                                )
                            )
                        }
                    }
                }
            }
            CurrentInvestmentStrategyDto::OtherInvestmentStrategy(other_investment_strategy_dto) => {
                Ok(CurrentInvestmentStrategy::OtherInvestmentStrategy(
                    other_investment_strategy_dto.try_into()?
                ))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GCWMInvestmentStrategies {
    PrimeModerate(CurrentInvestmentStrategyMonthYear)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum CurrentInvestmentStrategyMonthYear {
    Aug24
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherInvestmentStrategy {
    description: ConstrainedString200,
    fund_allocation: Option<FundHolding>
}

impl TryFrom<OtherInvestmentStrategyDto> for OtherInvestmentStrategy {
    type Error = String;

    fn try_from(dto: OtherInvestmentStrategyDto) -> Result<Self, Self::Error> {
        let validated_fund_allocation = match dto.fund_allocation {
            Some(unvalidated_fund_allocation) => {
                let validated_fund_holding: FundHolding = unvalidated_fund_allocation.try_into()?;
                Some(validated_fund_holding)
            }
            None => None
        };

        Ok(Self{
            description: dto.description.try_into()?,
            fund_allocation: validated_fund_allocation
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundHolding {
    fund_name: ConstrainedString200,
    isin: Option<ISIN>,
    sedol: Option<Sedol>,
    value: Option<ConstrainedMoneyAmountLarge>,
    percentage_of_portfolio: Option<Percentage>
}

impl TryFrom<FundHoldingDto> for FundHolding {
    type Error = String;

    fn try_from(dto: FundHoldingDto) -> Result<Self, Self::Error> {
        Ok(Self {
            fund_name: dto.fund_name.try_into()?,
            isin: if dto.isin.is_some() { Some(dto.isin.unwrap().try_into()?) } else { None },
            sedol: if dto.sedol.is_some() { Some(dto.sedol.unwrap().try_into()?) } else { None },
            value: if dto.value.is_some() { Some(dto.value.unwrap().try_into()?) } else { None },
            percentage_of_portfolio: if dto.percentage_of_portfolio.is_some() { Some(dto.percentage_of_portfolio.unwrap().try_into()?) } else { None },
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Valuation {
    value: ConstrainedMoneyAmountLarge,
    date_of_valuation: Date
}

impl TryFrom<ValuationDto> for Valuation {
    type Error = String;

    fn try_from(dto: ValuationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            date_of_valuation: dto.date_of_valuation.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductCharges {
    ongoing_advice_charge: Percentage,
    platform_charge: Percentage,
    ongoing_fund_charge: Percentage,
    other_charges: OtherCharge
}

impl TryFrom<ProductChargesDto> for ProductCharges {
    type Error = String;

    fn try_from(dto: ProductChargesDto) -> Result<Self, Self::Error> {
        Ok(Self {
            ongoing_advice_charge: dto.ongoing_advice_charge.try_into()?,
            platform_charge: dto.platform_charge.try_into()?,
            ongoing_fund_charge: dto.ongoing_fund_charge.try_into()?,
            other_charges: dto.other_charges.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentProductTaxPosition {
    product_tax_position: ProductTaxPosition
}

impl TryFrom<CurrentProductTaxPositionDto> for CurrentProductTaxPosition {
    type Error = String;

    fn try_from(dto: CurrentProductTaxPositionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            product_tax_position: dto.product_tax_position.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExistingProductRecommendations {
    recommended_product_charges: ProductCharges,
    product_retention: ProductRetention,
    recommended_investment_strategy: InvestableInvestmentStrategy,
    linked_objectives: Vec<Uuid>,
    recommendation_actions: Vec<RecommendedAction>
}

impl TryFrom<ExistingProductRecommendationsDto> for ExistingProductRecommendations {
    type Error = String;

    fn try_from(dto: ExistingProductRecommendationsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            recommended_product_charges: dto.recommended_product_charges.try_into()?,
            product_retention: dto.product_retention.try_into()?,
            recommended_investment_strategy: dto.recommended_investment_strategy.try_into()?,
            linked_objectives: dto.linked_objectives,
            recommendation_actions: dto.recommendation_actions.iter().map(|dto| dto.clone().try_into()).collect::<Result<_, _>>()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewProductRecommendations {
    recommended_product_charges: ProductCharges,
    recommended_investment_strategy: InvestableInvestmentStrategy,
    linked_objectives: Vec<Uuid>,
    recommendation_actions: Vec<RecommendedAction>
}

impl TryFrom<NewProductRecommendationsDto> for NewProductRecommendations {
    type Error = String;

    fn try_from(dto: NewProductRecommendationsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            recommended_product_charges: dto.recommended_product_charges.try_into()?,
            recommended_investment_strategy: dto.recommended_investment_strategy.try_into()?,
            linked_objectives: dto.linked_objectives,
            recommendation_actions: dto.recommendation_actions.iter().map(|dto| dto.clone().try_into()).collect::<Result<_, _>>()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherCharge {
    ongoing_charges: Option<Vec<OngoingCharge>>,
    incidental_charges: Option<Vec<IncidentalCharge>>
}

impl TryFrom<OtherChargeDto> for OtherCharge {
    type Error = String;

    fn try_from(dto: OtherChargeDto) -> Result<Self, Self::Error> {
        let ongoing_charges= dto.ongoing_charges
                                .map(|charges|
                                    charges.into_iter()
                                        .map(|dto| dto.try_into())
                                        .collect::<Result<Vec<_>, _>>()
                                )
                                .transpose()?;
        let incidental_charges = dto.incidental_charges
                                .map(|charges|
                                    charges.into_iter()
                                        .map(|dto| dto.try_into())
                                        .collect::<Result<Vec<_>, _>>()
                                )
                                .transpose()?;

        Ok(Self {
            ongoing_charges,
            incidental_charges
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedInvestmentAndRiskStrategy {
    recommended_investment_strategy: InvestableInvestmentStrategy,
    risk_level: RiskProfile
}

impl TryFrom<RecommendedInvestmentAndRiskStrategyDto> for RecommendedInvestmentAndRiskStrategy {
    type Error = String;

    fn try_from(dto: RecommendedInvestmentAndRiskStrategyDto) -> Result<Self, Self::Error> {
        Ok(Self {
            recommended_investment_strategy: dto.recommended_investment_strategy.try_into()?,
            risk_level: dto.risk_level.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum CapitalGainsPosition {
    CapitalGainsTaxAvoidLiability(CapitalGainsTaxAvoidLiability),
    CapitalGainsTaxNoLiability(CapitalGainsTaxNoLiability),
    CapitalGainsTaxIncurLiability(CapitalGainsTaxIncurLiability)
}

impl TryFrom<CapitalGainsPositionDto> for CapitalGainsPosition {
    type Error = String;

    fn try_from(dto: CapitalGainsPositionDto) -> Result<Self, Self::Error> {
        match dto {
            CapitalGainsPositionDto::CapitalGainsTaxAvoidLiability(capital_gains_tax_avoid_liability_dto) => {
                Ok(Self::CapitalGainsTaxAvoidLiability(capital_gains_tax_avoid_liability_dto.try_into()?))
            }
            CapitalGainsPositionDto::CapitalGainsTaxNoLiability(capital_gains_tax_no_liability_dto) => {
                Ok(Self::CapitalGainsTaxNoLiability(capital_gains_tax_no_liability_dto.try_into()?))
            }
            CapitalGainsPositionDto::CapitalGainsTaxIncurLiability(capital_gains_tax_incur_liability_dto) => {
                Ok(Self::CapitalGainsTaxIncurLiability(capital_gains_tax_incur_liability_dto.try_into()?))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChargeableGainsPosition {
    ChargeableGainsTaxAvoidLiability(ChargeableGainsTaxAvoidLiability),
    ChargeableGainsTaxNoLiability(ChargeableGainsTaxNoLiability),
    ChargeableGainsTaxIncurLiability(ChargeableGainsTaxIncurLiability)
}

impl TryFrom<ChargeableGainsPositionDto> for ChargeableGainsPosition {
    type Error = String;

    fn try_from(dto: ChargeableGainsPositionDto) -> Result<Self, Self::Error> {
        match dto {
            ChargeableGainsPositionDto::ChargeableGainsTaxAvoidLiability(capital_gains_tax_avoid_liability_dto) => {
                Ok(Self::ChargeableGainsTaxAvoidLiability(capital_gains_tax_avoid_liability_dto.try_into()?))
            }
            ChargeableGainsPositionDto::ChargeableGainsTaxNoLiability(capital_gains_tax_no_liability_dto) => {
                Ok(Self::ChargeableGainsTaxNoLiability(capital_gains_tax_no_liability_dto.try_into()?))
            }
            ChargeableGainsPositionDto::ChargeableGainsTaxIncurLiability(capital_gains_tax_incur_liability_dto) => {
                Ok(Self::ChargeableGainsTaxIncurLiability(capital_gains_tax_incur_liability_dto.try_into()?))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ProductRetention {
    Retain(Retain),
    Replace(Replace), 
    FullyEncash(FullyEncash),
    PartialTransfer(PartialTransfer)
}

impl TryFrom<ProductRetentionDto> for ProductRetention {
    type Error = String;

    fn try_from(dto: ProductRetentionDto) -> Result<Self, Self::Error> {
        match dto {
            ProductRetentionDto::Retain(retain_dto) => Ok(Self::Retain(retain_dto.try_into()?)),
            ProductRetentionDto::Replace(replace_dto) => Ok(Self::Replace(replace_dto.try_into()?)),
            ProductRetentionDto::FullyEncash(fully_encash_dto) => Ok(Self::FullyEncash(fully_encash_dto.try_into()?)),
            ProductRetentionDto::PartialTransfer(partial_transfer_dto) => Ok(Self::PartialTransfer(partial_transfer_dto.try_into()?)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RecommendedAction {
    SingleWithdrawal(SingleWithdrawal),
    SingleContribution(SingleContribution),
    RegularContribution(RegularContribution),
    RegularWithdrawal(RegularWithdrawal),
    Transfer(Transfer),
    StopWithdrawal(StopWithdrawal)
}

impl TryFrom<RecommendedActionDto> for RecommendedAction {
    type Error = String;

    fn try_from(dto: RecommendedActionDto) -> Result<Self, Self::Error> {
        match dto {
            RecommendedActionDto::SingleWithdrawal(single_withdrawal_dto) => Ok(Self::SingleWithdrawal(single_withdrawal_dto.try_into()?)),
            RecommendedActionDto::SingleContribution(single_contribution_dto) => Ok(Self::SingleContribution(single_contribution_dto.try_into()?)),
            RecommendedActionDto::RegularContribution(regular_contribution_dto) => Ok(Self::RegularContribution(regular_contribution_dto.try_into()?)),
            RecommendedActionDto::RegularWithdrawal(regular_withdrawal_dto) => Ok(Self::RegularWithdrawal(regular_withdrawal_dto.try_into()?)),
            RecommendedActionDto::StopWithdrawal(stop_withdrawal_dto) => Ok(Self::StopWithdrawal(stop_withdrawal_dto.try_into()?)),
            RecommendedActionDto::Transfer(transfer_dto) => Ok(Self::Transfer(transfer_dto.try_into()?)),
            RecommendedActionDto::StopWithdrawal(stop_withdrawal_dto) => Ok(Self::StopWithdrawal(stop_withdrawal_dto.try_into()?))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ProductTaxPosition {
    CapitalGainsTaxPosition(CapitalGainsPosition),
    ChargeableGainsPosition(ChargeableGainsPosition)
}

impl TryFrom<ProductTaxPositionDto> for ProductTaxPosition {
    type Error = String;

    fn try_from(dto: ProductTaxPositionDto) -> Result<Self, Self::Error> {
        match dto {
            ProductTaxPositionDto::CapitalGainsTaxPositionDto(capital_gains_tax_position_dto) => {
                Ok(Self::CapitalGainsTaxPosition(
                    capital_gains_tax_position_dto.try_into()?
                ))
            }
            ProductTaxPositionDto::ChargeableGainsPositionDto(chargeable_gains_tax_position_dto) => {
                Ok(Self::ChargeableGainsPosition(
                    chargeable_gains_tax_position_dto.try_into()?
                ))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OngoingCharge {
    charge_description: ConstrainedString200,
    charge_value: ConstrainedMoneyAmountSmall,
    frequency: Frequency
}

impl TryFrom<OngoingChargeDto> for OngoingCharge {
    type Error = String;

    fn try_from(dto: OngoingChargeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            charge_description: dto.charge_description.try_into()?,
            charge_value: dto.charge_value.try_into()?,
            frequency: dto.frequency.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncidentalCharge {
    charge_description: ConstrainedString200,
    charge_value: ConstrainedMoneyAmountSmall,
    frequency: Frequency,
    trigger_event: ConstrainedString200
}

impl TryFrom<IncidentalChargeDto> for IncidentalCharge {
    type Error = String;

    fn try_from(dto: IncidentalChargeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            charge_description: dto.charge_description.try_into()?,
            charge_value: dto.charge_value.try_into()?,
            frequency: dto.frequency.try_into()?,
            trigger_event: dto.trigger_event.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxAvoidLiability {
    unrealised_gains: ConstrainedMoneyAmountMedium,
    capital_gains_tax_discussion: ConstrainedString1000
}

impl TryFrom<CapitalGainsTaxAvoidLiabilityDto> for CapitalGainsTaxAvoidLiability {
    type Error = String;

    fn try_from(dto: CapitalGainsTaxAvoidLiabilityDto) -> Result<Self, Self::Error> {
        Ok(Self {
            unrealised_gains: dto.unrealised_gains.try_into()?,
            capital_gains_tax_discussion: dto.capital_gains_tax_discussion.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxNoLiability {
    unrealised_gains: ConstrainedMoneyAmountMedium,
    capital_gains_tax_discussion: ConstrainedString1000
}

impl TryFrom<CapitalGainsTaxIncurLiabilityDto> for CapitalGainsTaxIncurLiability {
    type Error = String;

    fn try_from(dto: CapitalGainsTaxIncurLiabilityDto) -> Result<Self, Self::Error> {
        Ok(Self {
            unrealised_gains: dto.unrealised_gains.try_into()?,
            capital_gains_tax_discussion: dto.capital_gains_tax_discussion.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGainsTaxIncurLiability {
    unrealised_gains: ConstrainedMoneyAmountMedium,
    capital_gains_tax_discussion: ConstrainedString1000
}

impl TryFrom<CapitalGainsTaxNoLiabilityDto> for CapitalGainsTaxNoLiability {
    type Error = String;
    
    fn try_from(dto: CapitalGainsTaxNoLiabilityDto) -> Result<Self, Self::Error> {
        Ok(Self {
            unrealised_gains: dto.unrealised_gains.try_into()?,
            capital_gains_tax_discussion: dto.capital_gains_tax_discussion.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxAvoidLiability {
    unrealised_gains: ConstrainedMoneyAmountMedium,
    chargeable_gains_tax_discussion: ConstrainedString1000
}

impl TryFrom<ChargeableGainsTaxAvoidLiabilityDto> for ChargeableGainsTaxAvoidLiability {
    type Error = String;
    
    fn try_from(dto: ChargeableGainsTaxAvoidLiabilityDto) -> Result<Self, Self::Error> {
        Ok(Self {
            unrealised_gains: dto.unrealised_gains.try_into()?,
            chargeable_gains_tax_discussion: dto.chargeable_gains_tax_discussion.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxNoLiability {
    unrealised_gains: ConstrainedMoneyAmountMedium,
    chargeable_gains_tax_discussion: ConstrainedString1000
}

impl TryFrom<ChargeableGainsTaxNoLiabilityDto> for ChargeableGainsTaxNoLiability {
    type Error = String;
    
    fn try_from(dto: ChargeableGainsTaxNoLiabilityDto) -> Result<Self, Self::Error> {
        Ok(Self {
            unrealised_gains: dto.unrealised_gains.try_into()?,
            chargeable_gains_tax_discussion: dto.chargeable_gains_tax_discussion.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeableGainsTaxIncurLiability {
    unrealised_gains: ConstrainedMoneyAmountMedium,
    chargeable_gains_tax_discussion: ConstrainedString1000
}

impl TryFrom<ChargeableGainsTaxIncurLiabilityDto> for ChargeableGainsTaxIncurLiability {
    type Error = String;
    
    fn try_from(dto: ChargeableGainsTaxIncurLiabilityDto) -> Result<Self, Self::Error> {
        Ok(Self {
            unrealised_gains: dto.unrealised_gains.try_into()?,
            chargeable_gains_tax_discussion: dto.chargeable_gains_tax_discussion.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Retain {
    rationale: ConstrainedString1000,
}

impl TryFrom<RetainDto> for Retain {
    type Error = String;

    fn try_from(dto: RetainDto) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Replace {
    rationale: ConstrainedString1000,
    replacement_product_information: ReplacementProductInformation
}

impl TryFrom<ReplaceDto> for Replace {
    type Error = String;

    fn try_from(dto: ReplaceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?,
            replacement_product_information: dto.replacement_product_information.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartialTransfer {
    rationale: ConstrainedString1000,
    value_to_transfer: ConstrainedMoneyAmountLarge,
    replacement_product_information: ReplacementProductInformation
}

impl TryFrom<PartialTransferDto> for PartialTransfer {
    type Error = String;

    fn try_from(dto: PartialTransferDto) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?,
            value_to_transfer: dto.value_to_transfer.try_into()?,
            replacement_product_information: dto.replacement_product_information.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullyEncash {
    rationale: ConstrainedString1000,
}

impl TryFrom<FullyEncashDto> for FullyEncash {
    type Error = String;

    fn try_from(dto: FullyEncashDto) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingleWithdrawal {
    value: ConstrainedMoneyAmountLarge,
    executive_summary_description: ConstrainedString200,
    rationale: ConstrainedString1000,
    date_of_action: Option<Date>,
    tax_year_of_action: Option<Date>
}

impl TryFrom<SingleWithdrawalDto> for SingleWithdrawal {
    type Error = String;

    fn try_from(dto: SingleWithdrawalDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description: dto.executive_summary_description.try_into()?,
            rationale: dto.rationale.try_into()?,
            date_of_action: if dto.date_of_action.is_some() { Some(dto.date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None }
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingleContribution {
    value: ConstrainedMoneyAmountLarge,
    executive_summary_description: ConstrainedString200,
    rationale: ConstrainedString1000,
    date_of_action: Option<Date>,
    tax_year_of_action: Option<TaxYear>
}

impl TryFrom<SingleContributionDto> for SingleContribution {
    type Error = String;

    fn try_from(dto: SingleContributionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description: dto.executive_summary_description.try_into()?,
            rationale: dto.rationale.try_into()?,
            date_of_action: if dto.date_of_action.is_some() { Some(dto.date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None }
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegularContribution {
    value: ConstrainedMoneyAmountLarge,
    executive_summary_description: ConstrainedString200,
    rationale: ConstrainedString1000,
    start_date_of_action: Date,
    frequency: Frequency,
    tax_year_of_action: Option<TaxYear>,
    end_date_of_action: Option<Date>,
}

impl TryFrom<RegularContributionDto> for RegularContribution {
    type Error = String;

    fn try_from(dto: RegularContributionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description: dto.executive_summary_description.try_into()?,
            rationale: dto.rationale.try_into()?,
            start_date_of_action: dto.start_date_of_action.try_into()?,
            frequency: dto.frequency.try_into()?,
            end_date_of_action: if dto.end_date_of_action.is_some() { Some(dto.end_date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None }
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegularWithdrawal {
    value: ConstrainedMoneyAmountLarge,
    executive_summary_description: ConstrainedString200,
    rationale: ConstrainedString1000,
    frequency: Frequency,
    start_date_of_action: Date,
    tax_year_of_action: Option<TaxYear>,
    end_date_of_action: Option<Date>,
}

impl TryFrom<RegularWithdrawalDto> for RegularWithdrawal {
    type Error = String;

    fn try_from(dto: RegularWithdrawalDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description: dto.executive_summary_description.try_into()?,
            rationale: dto.rationale.try_into()?,
            start_date_of_action: dto.start_date_of_action.try_into()?,
            frequency: dto.frequency.try_into()?,
            end_date_of_action: if dto.end_date_of_action.is_some() { Some(dto.end_date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None }
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    value: ConstrainedMoneyAmountLarge,
    executive_summary_description: ConstrainedString200,
    rationale: ConstrainedString1000,
    date_of_action: Option<Date>,
    tax_year_of_action: Option<TaxYear>,
    transfer_to_details: Vec<TransferDetail>
}

impl TryFrom<TransferDto> for Transfer {
    type Error = String;

    fn try_from(dto: TransferDto) -> Result<Self, Self::Error> {

        let transfer_to_details = dto.transfer_to_details
                                                                .iter()
                                                                .map(|dto| dto.clone().try_into())
                                                                .collect::<Result<_,_>>()?;

        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description: dto.executive_summary_description.try_into()?,
            rationale: dto.rationale.try_into()?,
            date_of_action: if dto.date_of_action.is_some() { Some(dto.date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None },
            transfer_to_details
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopWithdrawal {
    value: ConstrainedMoneyAmountLarge,
    executive_summary_description: ConstrainedString200,
    rationale: ConstrainedString1000,
    start_date_of_action: Option<Date>,
    tax_year_of_action: Option<TaxYear>,
    end_date_of_action: Option<Date>,
}

impl TryFrom<StopWithdrawalDto> for StopWithdrawal {
    type Error = String;

    fn try_from(dto: StopWithdrawalDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description: dto.executive_summary_description.try_into()?,
            rationale: dto.rationale.try_into()?,
            start_date_of_action: if dto.start_date_of_action.is_some() { Some(dto.start_date_of_action.unwrap().try_into()?) } else { None },
            end_date_of_action: if dto.end_date_of_action.is_some() { Some(dto.end_date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None }
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvestableInvestmentStrategy {
    risk_level: RiskProfile,
    fund_allocations: BespokeOrFirmInvestmentStrategy
}

impl TryFrom<InvestableInvestmentStrategyDto> for InvestableInvestmentStrategy {
    type Error = String;

    fn try_from(dto: InvestableInvestmentStrategyDto) -> Result<Self, Self::Error> {
        Ok(Self {
            risk_level: dto.risk_level.try_into()?,
            fund_allocations: dto.fund_allocations.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferDetail {
    value: ConstrainedMoneyAmountLarge,
    transfer_to_provider: Provider,
    transfer_to_tax_wrapper: TaxWrapperType,
    transfer_to_account_or_reference_number: Option<AccountOrReferenceNumberType>
}

impl TryFrom<TransferDetailDto> for TransferDetail {
    type Error = String;

    fn try_from(dto: TransferDetailDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            transfer_to_account_or_reference_number: dto.transfer_to_account_or_reference_number.map(|dto| dto.try_into()).transpose()?,
            transfer_to_provider: dto.transfer_to_provider.try_into()?,
            transfer_to_tax_wrapper: dto.transfer_to_tax_wrapper.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReplacementProductInformation {
    PensionReplacementProductInformation(PensionReplacementProductInformation),
    InvestmentReplacementProductInformation(InvestmentReplacementProductInformation)
}

impl TryFrom<ReplacementProductInformationDto> for ReplacementProductInformation {
    type Error = String;

    fn try_from(dto: ReplacementProductInformationDto) -> Result<Self, Self::Error> {
        match dto {
            ReplacementProductInformationDto::PensionReplacementProductInformation(pension_replacement_product_information_dto) => {
                Ok(Self::PensionReplacementProductInformation(pension_replacement_product_information_dto.try_into()?))
            }
            ReplacementProductInformationDto::InvestmentReplacementProductInformation(investment_replacement_product_information_dto) => {
                Ok(Self::InvestmentReplacementProductInformation(investment_replacement_product_information_dto.try_into()?))
            }
        }
    }
} 

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BespokeOrFirmInvestmentStrategy {
    BespokeInvestmentStrategy(BespokeInvestmentStrategy),
    FirmInvestmentStrategy(PresentFirmInvestmentStrategy)
}

impl TryFrom<BespokeOrFirmInvestmentStrategyDto> for BespokeOrFirmInvestmentStrategy {
    type Error = String;

    fn try_from(dto: BespokeOrFirmInvestmentStrategyDto) -> Result<Self, Self::Error> {
        match dto {
            BespokeOrFirmInvestmentStrategyDto::BespokeInvestmentStrategy(bespoke_investment_strategy_dto) => {
                Ok(Self::BespokeInvestmentStrategy(bespoke_investment_strategy_dto.try_into()?))
            }
            BespokeOrFirmInvestmentStrategyDto::FirmInvestmentStrategy(firm_investment_strategy_dto) => {
                Ok(Self::FirmInvestmentStrategy(firm_investment_strategy_dto.try_into()?))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BespokeInvestmentStrategy {
    description: ConstrainedString200,
    fund_allocation: Option<FundHolding>
}

impl TryFrom<BespokeInvestmentStrategyDto> for BespokeInvestmentStrategy {
    type Error = String;

    fn try_from(dto: BespokeInvestmentStrategyDto) -> Result<Self, Self::Error> {
        Ok(Self {
            description: dto.description.try_into()?,
            fund_allocation: if dto.fund_allocation.is_some() { Some(dto.fund_allocation.unwrap().try_into()?) } else { None }
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PresentFirmInvestmentStrategy {
    PrimeModerate(Vec<FundHolding>)
}

impl TryFrom<PresentFirmInvestmentStrategyDto> for PresentFirmInvestmentStrategy {
    type Error = String;

    fn try_from(dto: PresentFirmInvestmentStrategyDto) -> Result<Self, Self::Error> {
        match dto {
            PresentFirmInvestmentStrategyDto::PrimeModerate(fund_holdings_dto) => {
                Ok(Self::PrimeModerate(
                    fund_holdings_dto.iter().map(|fund_holding_dto| fund_holding_dto.clone().try_into()).collect::<Result<_,_>>()?
                ))
            }
        }
    }
} 

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PensionReplacementProductInformation {
    start_date: Date,
    total_contributions: ConstrainedMoneyAmountLarge,
    current_transfer_value: ConstrainedMoneyAmountLarge,
    no_of_funds_available: i32,
    max_number_of_funds_invested_at_one_time: Option<i32>,
    retirement_date_age: ProductRetirementAge,
    is_waiver_of_premium_insurance_available: bool,
    death_benefits_description: ConstrainedString200,
    is_life_cover_available: Option<ConstrainedMoneyAmountLarge>,
    loyalty_bonus: Option<Percentage>,
    fund_bonus_enhanced_allocation: Option<FundBonusAllocation>,
    tax_free_cash_entitlement: ProtectedTaxFreeCashEntitlement,
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
    is_guaranteed_minimum_pension_or_reference_scheme_test: bool,
    is_guaranteed_annuity_rates: bool,
    other_features: Vec<(ConstrainedString200, ConstrainedString1000)>
}

impl TryFrom<PensionReplacementProductInformationDto> for PensionReplacementProductInformation {
    type Error = String;

    fn try_from(dto: PensionReplacementProductInformationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            start_date: dto.start_date.try_into()?,
            total_contributions: dto.total_contributions.try_into()?,
            current_transfer_value: dto.current_transfer_value.try_into()?,
            no_of_funds_available: dto.no_of_funds_available,
            max_number_of_funds_invested_at_one_time: if dto.max_number_of_funds_invested_at_one_time.is_some() { dto.max_number_of_funds_invested_at_one_time } else { None },
            retirement_date_age: dto.retirement_date_age.try_into()?,
            is_waiver_of_premium_insurance_available: dto.is_waiver_of_premium_insurance_available,
            death_benefits_description: dto.death_benefits_description.try_into()?,
            is_life_cover_available: if dto.is_life_cover_available.is_some() { Some(dto.is_life_cover_available.unwrap().try_into()?) } else { None },
            loyalty_bonus: if dto.loyalty_bonus.is_some() { Some(dto.loyalty_bonus.unwrap().try_into()?) } else { None },
            fund_bonus_enhanced_allocation: if dto.fund_bonus_enhanced_allocation.is_some() { Some(dto.fund_bonus_enhanced_allocation.unwrap().try_into()?) } else { None },
            tax_free_cash_entitlement: dto.tax_free_cash_entitlement.try_into()?,
            is_flexi_access_available: dto.is_flexi_access_available,
            is_full_ufpls_available: dto.is_full_ufpls_available,
            is_partial_ufpls_available: dto.is_partial_ufpls_available,
            is_transfers_contributions_allowed_in: dto.is_transfers_contributions_allowed_in,
            is_block_or_bulk_transfer_received: dto.is_block_or_bulk_transfer_received,
            is_enhanced_protection_available: dto.is_enhanced_protection_available,
            is_earmarking_order: dto.is_earmarking_order,
            is_charge_guarantee_and_guarantee_amount: dto.is_charge_guarantee_and_guarantee_amount,
            is_existing_pension_sharing_order: dto.is_existing_pension_sharing_order,
            is_guaranteed_minimum_fund: dto.is_guaranteed_minimum_fund,
            is_guaranteed_minimum_annuity: dto.is_guaranteed_minimum_annuity,
            is_guaranteed_minimum_pension_or_reference_scheme_test: dto.is_guaranteed_minimum_pension_or_reference_scheme_test,
            is_guaranteed_annuity_rates: dto.is_guaranteed_annuity_rates,
            other_features: dto.other_features.iter()
                .map(|(feature_name_dto, feature_explanation_dto)| {
                    Ok((
                        feature_name_dto.clone().try_into()?,
                        feature_explanation_dto.clone().try_into()?
                    ))
                })
                .collect::<Result<Vec<(ConstrainedString200, ConstrainedString1000) >, String>>()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentReplacementProductInformation {
    start_date: Date,
    total_contributions: ConstrainedMoneyAmountLarge,
    current_transfer_value: ConstrainedMoneyAmountLarge,
    no_of_funds_available: i32,
    max_number_of_funds_invested_at_one_time: Option<i32>,
    loyalty_bonus: Option<Percentage>,
    fund_bonus_enhanced_allocation: Option<FundBonusAllocation>,
    is_charge_guarantee_and_guarantee_amount: bool,
    is_guaranteed_return_applicable: bool,
    other_features: Vec<(ConstrainedString200, ConstrainedString1000)>
}

impl TryFrom<InvestmentReplacementProductInformationDto> for InvestmentReplacementProductInformation {
    type Error = String;

    fn try_from(dto: InvestmentReplacementProductInformationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            start_date: dto.start_date.try_into()?,
            total_contributions: dto.total_contributions.try_into()?,
            current_transfer_value: dto.current_transfer_value.try_into()?,
            no_of_funds_available: dto.no_of_funds_available,
            max_number_of_funds_invested_at_one_time: if dto.max_number_of_funds_invested_at_one_time.is_some() { dto.max_number_of_funds_invested_at_one_time } else { None },
            loyalty_bonus: dto.loyalty_bonus.map(|dto| dto.try_into()).transpose()?,
            fund_bonus_enhanced_allocation: dto.fund_bonus_enhanced_allocation.map(|dto| dto.try_into()).transpose()?,
            is_charge_guarantee_and_guarantee_amount: dto.is_charge_guarantee_and_guarantee_amount,
            is_guaranteed_return_applicable: dto.is_guaranteed_return_applicable,
            other_features: dto.other_features.iter()
                .map(|(feature_name_dto, feature_explanation_dto)| {
                    Ok((
                        feature_name_dto.clone().try_into()?,
                        feature_explanation_dto.clone().try_into()?
                    ))
                })
                .collect::<Result<Vec<(ConstrainedString200, ConstrainedString1000) >, String>>()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PlatformAccountNumberType {
    Abrdn(AbrdnAccountNumber),
    Transact(TransactPlatformNumber),
    Other(ConstrainedString200)
}

impl TryFrom<PlatformAccountNumberTypeDto> for PlatformAccountNumberType {
    type Error = String;

    fn try_from(dto: PlatformAccountNumberTypeDto) -> Result<Self, Self::Error> {
        match dto {
            PlatformAccountNumberTypeDto::Abrdn(abrdn_transact_number_dto) => {
                Ok(PlatformAccountNumberType::Abrdn(abrdn_transact_number_dto.try_into()?))
            }
            PlatformAccountNumberTypeDto::Transact(transact_number_dto) => {
                Ok(PlatformAccountNumberType::Transact(transact_number_dto.try_into()?))
            }
            PlatformAccountNumberTypeDto::Other(other_dto) => {
                Ok(PlatformAccountNumberType::Other(other_dto.try_into()?))
            }
        }
    }
}

impl fmt::Display for PlatformAccountNumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformAccountNumberType::Abrdn(account_number) => write!(f, "{}", account_number),
            PlatformAccountNumberType::Transact(transact_number) => write!(f, "{}", transact_number),
            PlatformAccountNumberType::Other(other) => write!(f, "{}", other),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum AccountOrReferenceNumberType {
    Abrdn(AbrdnFullAccountNumber),
    AbrdnSipp(AbrdnSippNumber),
    Transact(TransactReferenceNumber),
    Other(ConstrainedString200),
    NewAccount(Uuid)
}

impl TryFrom<AccountOrReferenceNumberTypeDto> for AccountOrReferenceNumberType {
    type Error = String;

    fn try_from(dto: AccountOrReferenceNumberTypeDto) -> Result<Self, Self::Error> {
        match dto {
            AccountOrReferenceNumberTypeDto::Abrdn(abrdn_full_account_number_dto) => {
                Ok(AccountOrReferenceNumberType::Abrdn(abrdn_full_account_number_dto.try_into()?))
            }
            AccountOrReferenceNumberTypeDto::AbrdnSipp(abrdn_sipp_number_dto) => {
                Ok(AccountOrReferenceNumberType::AbrdnSipp(abrdn_sipp_number_dto.try_into()?))
            }
            AccountOrReferenceNumberTypeDto::Transact(transact_reference_number_dto) => {
                Ok(AccountOrReferenceNumberType::Transact(transact_reference_number_dto.try_into()?))
            }
            AccountOrReferenceNumberTypeDto::Other(other_reference_number) => {
                Ok(AccountOrReferenceNumberType::Other(other_reference_number.try_into()?))
            }
            AccountOrReferenceNumberTypeDto::NewAccount(new_account_reference_number) => {
                Ok(AccountOrReferenceNumberType::NewAccount(Uuid::parse_str(&new_account_reference_number.as_str()).map_err(|err|format!("Failed to parse UUID: {}", err))?))
            }
        }
    }
}

impl fmt::Display for AccountOrReferenceNumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountOrReferenceNumberType::Abrdn(full_account_number) => write!(f, "{}", full_account_number),
            AccountOrReferenceNumberType::AbrdnSipp(sipp_number) => write!(f, "{}", sipp_number),
            AccountOrReferenceNumberType::Transact(transact_reference_number) => write!(f, "{}", transact_reference_number),
            AccountOrReferenceNumberType::Other(other_reference) => write!(f, "{}", other_reference),
            AccountOrReferenceNumberType::NewAccount(new_account_reference) => write!(f, "{}", new_account_reference),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PlatformOrAccountReferenceNumberType {
    PlatformAccountNumberType(PlatformAccountNumberType),
    AccountOrReferenceNumberType(AccountOrReferenceNumberType)
}

impl TryFrom<PlatformOrAccountReferenceNumberTypeDto> for PlatformOrAccountReferenceNumberType {
    type Error = String;

    fn try_from(dto: PlatformOrAccountReferenceNumberTypeDto) -> Result<Self, Self::Error> {
        match dto {
            PlatformOrAccountReferenceNumberTypeDto::PlatformAccountNumberType(platform_or_account_reference_number_dto) => {
                Ok(PlatformOrAccountReferenceNumberType::PlatformAccountNumberType(platform_or_account_reference_number_dto.try_into()?))
            }
            PlatformOrAccountReferenceNumberTypeDto::AccountOrReferenceNumberType(account_number_reference_number_dto) => {
                Ok(PlatformOrAccountReferenceNumberType::AccountOrReferenceNumberType(account_number_reference_number_dto.try_into()?))
            }
        }
    }
}