use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::convert::TryFrom;
use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;

use crate::domain::constrained_types::abrdn_account_number::AbrdnAccountNumber;
use crate::domain::constrained_types::abrdn_full_account_number::AbrdnFullAccountNumber;
use crate::domain::constrained_types::abrdn_sipp_number::AbrdnSippNumber;
use crate::domain::constrained_types::bank_account_numbers::{BankSortCode, BankAccountNumber};
use crate::domain::constrained_types::retirement_age::InvalidAgeError;
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
use crate::driven::repository::InvestmentPortfoliosRepository;
use super::investment_holdings::{FundHolding, InvestmentPortfolio, InvestmentStrategy};
use super::risk_assessment::RiskProfile;
use crate::driving::data_transfer_object::report_type_data_transfer_object::product::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Products(Vec<ExistingNewJointSingleProduct>);

impl Products {

    pub fn value(&self) -> &Vec<ExistingNewJointSingleProduct> {
        &self.0
    }

    pub async fn from_dto<R>(
        dto: ProductsDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        let mut out = Vec::with_capacity(dto.value().len());
        for item_dto in dto.value().iter().cloned() {
            // await each conversion in turn, propagating errors
            let product = ExistingNewJointSingleProduct::from_dto(item_dto, repo).await?;
            out.push(product);
        }
        Ok(Products(out))
    }

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

    /// Returns a HashMap mapping account or reference numbers to their corresponding products.
    pub fn products_by_account_number_or_new_product_id(&self) -> HashMap<String, &ExistingNewJointSingleProduct> {
        let mut map = HashMap::new();

        for product in &self.0 {
            let account_number = match product {
                ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(p) => p.account_or_reference_number.to_string(),
                ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(p) => p.account_or_reference_number.to_string(),
                ExistingNewJointSingleProduct::NewSingleOwnedProduct(p) => {
                    match &p.account_or_reference_number { 
                        Some(account_number) => account_number.to_string(),
                        None => p.id.to_string()
                    }
                }
            };

            map.insert(account_number, product);
        }

        map
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
            ExistingProduct::JointlyOwned(product) => {
                match &product.account_type {
                    CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(gia) => gia.provider.0.to_string(),
                    CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(oib) => oib.provider.0.to_string(),
                    CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(oib) => oib.provider.0.to_string(),
                }
            }
            ExistingProduct::SingleOwned(product) => {
                match &product.account_type {
                    AccountType::IsaStocksAndShares(iss) => iss.provider.0.to_string(),
                    AccountType::SelfInvestedPersonalPension(sipp) => sipp.provider.0.to_string(),
                    AccountType::PersonalPension(pp) => pp.provider.0.to_string(),
                    AccountType::JuniorIsaStocksAndShares(jisa) => jisa.provider.0.to_string(),
                    AccountType::CashIsa(ci) => ci.provider.0.to_string(),
                    AccountType::GeneralInvestmentAccount(gia) => gia.provider.0.to_string(),
                    AccountType::OnshoreInvestmentBond(oib) => oib.provider.0.to_string(),
                    AccountType::OffshoreInvestmentBond(oib) => oib.provider.0.to_string(),
                }
            }
        }
    }

    /// Returns the account type as a string.
    pub fn account_type_as_string(&self) -> String {
        match self {
            ExistingProduct::JointlyOwned(product) => product.account_type.to_string(),
            ExistingProduct::SingleOwned(product) => product.account_type.to_string(),
        }
    }

    /// Returns the account type as a string with brackets short name.
    pub fn account_type_as_full_name_brackets_string_short_name(&self) -> String {
        match self {
            ExistingProduct::JointlyOwned(product) => {
                product.account_type.account_type_as_full_name_brackets_string_short_name()
            }
            ExistingProduct::SingleOwned(product) => {
                product.account_type.account_type_as_full_name_brackets_string_short_name()
            }
        }
    }

    /// Returns the platform or account number.
    /// The platform or account number is a higher level account number within which numerous account or reference numbers can be 
    /// held which are unique identifiers to the individual tax wrappers themselves.
    pub fn platform_account_number(&self) -> &Option<PlatformAccountNumberType> {
        match self {
            ExistingProduct::JointlyOwned(product) => &product.platform_or_account_number,
            ExistingProduct::SingleOwned(product) => &product.platform_or_account_number,
        }
    }

    /// Returns the platform or account number as a string.
    pub fn platform_account_number_as_string(&self) -> Option<String> {
        match self {
            ExistingProduct::JointlyOwned(product) => product.platform_or_account_number.as_ref().map(|platform_number| platform_number.to_string()),
            ExistingProduct::SingleOwned(product) => product.platform_or_account_number.as_ref().map(|platform_number| platform_number.to_string()),
        }
    }

    /// Returns the account or reference number as a string.
    /// The unique identifier for each tax wrapper held within a platform / wrap account.
    /// For new accounts where an account or reference number is not provided a temporary uuid is used in place.
    pub fn account_or_reference_number(&self) -> &AccountOrReferenceNumberType {
        match self {
            ExistingProduct::JointlyOwned(product) => &product.account_or_reference_number,
            ExistingProduct::SingleOwned(product) => &product.account_or_reference_number,
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
            ExistingProduct::JointlyOwned(product) => {
                match &product.account_type {
                    CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(gia) => &gia.provider,
                    CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(oib) => &oib.provider,
                    CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(oib) => &oib.provider,
                }
            }
            ExistingProduct::SingleOwned(product) => {
                match &product.account_type {
                    AccountType::IsaStocksAndShares(iss) => &iss.provider,
                    AccountType::SelfInvestedPersonalPension(sipp) => &sipp.provider,
                    AccountType::PersonalPension(pp) => &pp.provider,
                    AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.provider,
                    AccountType::CashIsa(ci) => &ci.provider,
                    AccountType::GeneralInvestmentAccount(gia) => &gia.provider,
                    AccountType::OnshoreInvestmentBond(oib) => &oib.provider,
                    AccountType::OffshoreInvestmentBond(oib) => &oib.provider,
                }
            }
        }
    }

    /// Returns a reference to the retention recommendation.
    pub fn product_retention(&self) -> &ProductRetention {
        match self {
            ExistingProduct::JointlyOwned(product) => {
                match &product.account_type {
                    CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(gia) => &gia.recommendations.product_retention,
                    CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
                    CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
                }
            }
            ExistingProduct::SingleOwned(product) => {
                match &product.account_type {
                    AccountType::IsaStocksAndShares(iss) => &iss.recommendations.product_retention,
                    AccountType::SelfInvestedPersonalPension(sipp) => &sipp.recommendations.product_retention,
                    AccountType::PersonalPension(pp) => &pp.recommendations.product_retention,
                    AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.recommendations.product_retention,
                    AccountType::CashIsa(ci) => &ci.recommendations.product_retention,
                    AccountType::GeneralInvestmentAccount(gia) => &gia.recommendations.product_retention,
                    AccountType::OnshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
                    AccountType::OffshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
                }
            }
        }
    }

    /// Returns a reference to the rationale stored in the product retention recommendations.
    pub fn rationale(&self) -> &ConstrainedString1000 {
        let retention = self.product_retention();
        match retention {
            ProductRetention::Retain(retain) => &retain.rationale,
            ProductRetention::Replace(replace) => {
                match replace {
                    Replace::FullyReplace(fully) => &fully.rationale,
                    Replace::PartiallyReplace(partial) => &partial.rationale,
                }
            }
            ProductRetention::FullyEncash(encash) => &encash.rationale,
        }
    }

    /// Returns a reference to the linked objectives (a vector of Uuid) if available.
    /// If the underlying product retention is not of type Retain, returns None.
    pub fn linked_objectives(&self) -> Option<&Vec<Uuid>> {
        // Retrieve the product retention from the inner product.
        let retention = self.product_retention();
        if let ProductRetention::Retain(retain) = retention {
            Some(&retain.linked_objectives)
        } else {
            None
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
            NewProduct::SingleOwned(product) => {
                match &product.account_type {
                    AccountType::IsaStocksAndShares(iss) => iss.provider.0.to_string(),
                    AccountType::SelfInvestedPersonalPension(sipp) => sipp.provider.0.to_string(),
                    AccountType::PersonalPension(pp) => pp.provider.0.to_string(),
                    AccountType::JuniorIsaStocksAndShares(jisa) => jisa.provider.0.to_string(),
                    AccountType::CashIsa(ci) => ci.provider.0.to_string(),
                    AccountType::GeneralInvestmentAccount(gia) => gia.provider.0.to_string(),
                    AccountType::OnshoreInvestmentBond(oib) => oib.provider.0.to_string(),
                    AccountType::OffshoreInvestmentBond(oib) => oib.provider.0.to_string(),
                }
            }
        }
    }

    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        match self {
            NewProduct::SingleOwned(product) => product.account_type.to_string(),
        }
    }

    /// Returns the platform or account number.
    /// The platform or account number is a higher level account number within which numerous account or reference numbers can be 
    /// held which are unique identifiers to the individual tax wrappers themselves.
    pub fn platform_account_number(&self) -> &Option<PlatformAccountNumberType> {
        match self {
            NewProduct::SingleOwned(product) => &product.platform_or_account_number,
        }
    }

    /// Returns the account or reference number as a string.
    /// The unique identifier for each tax wrapper held within a platform / wrap account.
    /// For new accounts where an account or reference number is not provided a temporary uuid is used in place.
    pub fn account_or_reference_number_or_id_as_string(&self) -> String {
        match self {
            NewProduct::SingleOwned(product) => {
                match &product.account_or_reference_number { 
                    Some(account_number) => account_number.to_string(),
                    None => product.id.to_string()
                }
            }
        }
    }

    pub fn account_or_reference_number(&self) -> &Option<AccountOrReferenceNumberType> {
        match self {
            NewProduct::SingleOwned(product) => {
                &product.account_or_reference_number
            }
        }
    }

    /// Returns a reference to the provider.
    pub fn provider(&self) -> &Provider {
        match self {
            NewProduct::SingleOwned(product) => {
                match &product.account_type {
                    AccountType::IsaStocksAndShares(iss) => &iss.provider,
                    AccountType::SelfInvestedPersonalPension(sipp) => &sipp.provider,
                    AccountType::PersonalPension(pp) => &pp.provider,
                    AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.provider,
                    AccountType::CashIsa(ci) => &ci.provider,
                    AccountType::GeneralInvestmentAccount(gia) => &gia.provider,
                    AccountType::OnshoreInvestmentBond(oib) => &oib.provider,
                    AccountType::OffshoreInvestmentBond(oib) => &oib.provider,
                }
            }
        }
    }

    /// Returns a reference to the rationale stored in the new product recommendations.
    pub fn rationale(&self) -> &ConstrainedString1000 {
        match self {
            NewProduct::SingleOwned(product) => &product.recommendations.rationale,
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

impl ExistingNewJointSingleProduct {

    pub async fn from_dto<R>(
        dto: ExistingNewJointSingleProductDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        let product = match dto {
            ExistingNewJointSingleProductDto::ExistingJointlyOwnedProduct(inner_dto) => {
                // call the async constructor, await, propagate any error with `?`
                let inner = ExistingJointlyOwnedProduct::from_dto(inner_dto, repo).await?;
                Self::ExistingJointlyOwnedProduct(inner)
            }
            ExistingNewJointSingleProductDto::ExistingSingleOwnedProduct(inner_dto) => {
                let inner = ExistingSingleOwnedProduct::from_dto(inner_dto, repo).await?;
                Self::ExistingSingleOwnedProduct(inner)
            }
            ExistingNewJointSingleProductDto::NewSingleOwnedProduct(inner_dto) => {
                let inner = NewSingleOwnedProduct::from_dto(inner_dto, repo).await?;
                Self::NewSingleOwnedProduct(inner)
            }
        };

        Ok(product)
    }
}

impl ExistingNewJointSingleProduct {
    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                product.account_type.to_string()
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                product.account_type.to_string()
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                product.account_type.to_string()
            }
        }
    }

    /// Returns the account, reference number or id of new product as a string.
    pub fn account_or_reference_number_as_string(&self) -> String {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                product.account_or_reference_number.to_string()
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                product.account_or_reference_number.to_string()
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                product.account_or_reference_number_or_id_as_string()
            }
        }
    }

    /// Returns the platform account number as a string.
    pub fn platform_account_number_as_string(&self) -> Option<String> {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                product.platform_or_account_number.as_ref().map(|platform_number| platform_number.to_string())
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                product.platform_or_account_number.as_ref().map(|platform_number| platform_number.to_string())
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                product.platform_or_account_number.as_ref().map(|num| num.to_string())
            }
        }
    }

    /// Returns a reference to the provider.
    pub fn provider(&self) -> &Provider {
        match self {
            ExistingNewJointSingleProduct::ExistingJointlyOwnedProduct(product) => {
                match &product.account_type {
                    CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(gia) => &gia.provider,
                    CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(oib) => &oib.provider,
                    CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(oib) => &oib.provider,
                }
            }
            ExistingNewJointSingleProduct::ExistingSingleOwnedProduct(product) => {
                match &product.account_type {
                    AccountType::IsaStocksAndShares(iss) => &iss.provider,
                    AccountType::SelfInvestedPersonalPension(sipp) => &sipp.provider,
                    AccountType::PersonalPension(pp) => &pp.provider,
                    AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.provider,
                    AccountType::CashIsa(ci) => &ci.provider,
                    AccountType::GeneralInvestmentAccount(gia) => &gia.provider,
                    AccountType::OnshoreInvestmentBond(oib) => &oib.provider,
                    AccountType::OffshoreInvestmentBond(oib) => &oib.provider,
                }
            }
            ExistingNewJointSingleProduct::NewSingleOwnedProduct(product) => {
                match &product.account_type {
                    AccountType::IsaStocksAndShares(iss) => &iss.provider,
                    AccountType::SelfInvestedPersonalPension(sipp) => &sipp.provider,
                    AccountType::PersonalPension(pp) => &pp.provider,
                    AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.provider,
                    AccountType::CashIsa(ci) => &ci.provider,
                    AccountType::GeneralInvestmentAccount(gia) => &gia.provider,
                    AccountType::OnshoreInvestmentBond(oib) => &oib.provider,
                    AccountType::OffshoreInvestmentBond(oib) => &oib.provider,
                }
            }
        }
    }

    /// Returns the provider as a string.
    pub fn provider_as_string(&self) -> String {
        self.provider().0.to_string()  // Access inner Providers enum
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExistingJointlyOwnedProduct {
    id: Uuid, 
    platform_or_account_number: Option<PlatformAccountNumberType>,
    account_or_reference_number: AccountOrReferenceNumberType,
    account_type: CanBeJointlyOwnedAccountType
}

impl ExistingJointlyOwnedProduct {

    pub async fn from_dto<R>(
        dto: ExistingJointlyOwnedProductDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(Self {
            id: Uuid::parse_str(&dto.id).map_err(|err|format!("Failed to parse UUID: {}", err))?,
            platform_or_account_number: dto.platform_or_account_number.map(|dto|dto.try_into()).transpose()?,
            account_or_reference_number: dto.account_or_reference_number.try_into()?,
            account_type: CanBeJointlyOwnedAccountType::from_dto(dto.account_type, repo).await?
        })
    }
}

impl ExistingJointlyOwnedProduct {
    /// Returns the provider as a string.
    pub fn provider_as_string(&self) -> String {
        match &self.account_type {
            CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(gia) => gia.provider.0.to_string(),
            CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(oib) => oib.provider.0.to_string(),
            CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(oib) => oib.provider.0.to_string(),
        }
    }

    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        self.account_type.to_string()
    }

    /// Returns the tax wrapper type as a full name/brackets string.
    pub fn tax_wrapper_type_as_full_name_brackets_string_short_name(&self) -> String {
        self.account_type.account_type_as_full_name_brackets_string_short_name()
    }

    /// Returns the platform or account number.
    /// The platform or account number is a higher level account number within which numerous account or reference numbers can be 
    /// held which are unique identifiers to the individual tax wrappers themselves.
    pub fn platform_account_number(&self) -> &Option<PlatformAccountNumberType> {
        &self.platform_or_account_number
    }

    /// Returns the platform or account number as a string.
    pub fn platform_account_number_as_string(&self) -> Option<String> {
        self.platform_or_account_number.as_ref().map(|platform_number| platform_number.to_string())
    }

    /// Returns the account or reference number as a string.
    /// The unique identifier for each tax wrapper held within a platform / wrap account.
    /// For new accounts where an account or reference number is not provided a temporary uuid is used in place.
    pub fn account_or_reference_number(&self) -> &AccountOrReferenceNumberType {
        &self.account_or_reference_number
    }

    /// Returns the account or reference number as a string.
    pub fn account_or_reference_number_as_string(&self) -> String {
        self.account_or_reference_number.to_string()
    }

    /// Returns a reference to the provider.
    pub fn provider(&self) -> &Provider {
        match &self.account_type {
            CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(gia) => &gia.provider,
            CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(oib) => &oib.provider,
            CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(oib) => &oib.provider,
        }
    }

    /// Returns a reference to the retention recommendation.
    pub fn product_retention(&self) -> &ProductRetention {
        match &self.account_type {
            CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(gia) => &gia.recommendations.product_retention,
            CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
            CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
        }
    }

    pub fn account_type(&self) -> &CanBeJointlyOwnedAccountType {
        &self.account_type
    }

}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExistingSingleOwnedProduct {
    id: Uuid,
    platform_or_account_number: Option<PlatformAccountNumberType>,
    account_or_reference_number: AccountOrReferenceNumberType,
    account_type: AccountType
}

impl ExistingSingleOwnedProduct {

    pub async fn from_dto<R>(
        dto: ExistingSingleOwnedProductDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(Self {
            id: Uuid::parse_str(&dto.id).map_err(|err|format!("Failed to parse UUID: {}", err))?,
            platform_or_account_number: dto.platform_or_account_number.map(|dto|dto.try_into()).transpose()?,
            account_or_reference_number: dto.account_or_reference_number.try_into()?,
            account_type: AccountType::from_dto(dto.account_type, repo).await?
        })
    }
}

impl ExistingSingleOwnedProduct {
    /// Returns the provider as a string.
    pub fn provider_as_string(&self) -> String {
        match &self.account_type {
            AccountType::IsaStocksAndShares(iss) => iss.provider.0.to_string(),
            AccountType::SelfInvestedPersonalPension(sipp) => sipp.provider.0.to_string(),
            AccountType::PersonalPension(pp) => pp.provider.0.to_string(),
            AccountType::JuniorIsaStocksAndShares(jisa) => jisa.provider.0.to_string(),
            AccountType::CashIsa(ci) => ci.provider.0.to_string(),
            AccountType::GeneralInvestmentAccount(gia) => gia.provider.0.to_string(),
            AccountType::OnshoreInvestmentBond(oib) => oib.provider.0.to_string(),
            AccountType::OffshoreInvestmentBond(oib) => oib.provider.0.to_string(),
        }
    }

    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        self.account_type.to_string()
    }

    /// Returns the tax wrapper type as a full name/brackets string.
    pub fn tax_wrapper_type_as_full_name_brackets_string_short_name(&self) -> String {
        self.account_type.account_type_as_full_name_brackets_string_short_name()  
    }

    /// Returns the platform or account number.
    /// The platform or account number is a higher level account number within which numerous account or reference numbers can be 
    /// held which are unique identifiers to the individual tax wrappers themselves.
    pub fn platform_account_number(&self) -> &Option<PlatformAccountNumberType> {
        &self.platform_or_account_number
    }

    /// Returns the platform or account number as a string.
    pub fn platform_account_number_as_string(&self) -> Option<String> {
        self.platform_or_account_number.as_ref().map(|number|number.to_string())
    }

    /// Returns the account or reference number as a string.
    /// The unique identifier for each tax wrapper held within a platform / wrap account.
    /// For new accounts where an account or reference number is not provided a temporary uuid is used in place.
    pub fn account_or_reference_number(&self) -> &AccountOrReferenceNumberType {
        &self.account_or_reference_number
    }

    /// Returns the account or reference number as a string.
    pub fn account_or_reference_number_as_string(&self) -> String {
        self.account_or_reference_number.to_string()
    }

    /// Returns a reference to the provider.
    pub fn provider(&self) -> &Provider {
        match &self.account_type {
            AccountType::IsaStocksAndShares(iss) => &iss.provider,
            AccountType::SelfInvestedPersonalPension(sipp) => &sipp.provider,
            AccountType::PersonalPension(pp) => &pp.provider,
            AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.provider,
            AccountType::CashIsa(ci) => &ci.provider,
            AccountType::GeneralInvestmentAccount(gia) => &gia.provider,
            AccountType::OnshoreInvestmentBond(oib) => &oib.provider,
            AccountType::OffshoreInvestmentBond(oib) => &oib.provider,
        }
    }

    /// Returns a reference to the retention recommendation.
    pub fn product_retention(&self) -> &ProductRetention {
        match &self.account_type {
            AccountType::IsaStocksAndShares(iss) => &iss.recommendations.product_retention,
            AccountType::SelfInvestedPersonalPension(sipp) => &sipp.recommendations.product_retention,
            AccountType::PersonalPension(pp) => &pp.recommendations.product_retention,
            AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.recommendations.product_retention,
            AccountType::CashIsa(ci) => &ci.recommendations.product_retention,
            AccountType::GeneralInvestmentAccount(gia) => &gia.recommendations.product_retention,
            AccountType::OnshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
            AccountType::OffshoreInvestmentBond(oib) => &oib.recommendations.product_retention,
        }
    }
    
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewSingleOwnedProduct {
    id: Uuid,
    platform_or_account_number: Option<PlatformAccountNumberType>,
    account_or_reference_number: Option<AccountOrReferenceNumberType>,
    account_type: AccountType,
    recommendations: NewProductRecommendations
}

impl NewSingleOwnedProduct {

    pub async fn from_dto<R>(
        dto: NewSingleOwnedProductDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(Self {
            id: Uuid::parse_str(&dto.id).map_err(|err|format!("Failed to parse UUID: {}", err))?,
            platform_or_account_number: dto.platform_or_account_number.map(|dto| dto.try_into()).transpose()?,
            account_or_reference_number: dto.account_or_reference_number.map(|dto|dto.try_into()).transpose()?,
            account_type: AccountType::from_dto(dto.account_type, repo).await?,
            recommendations: NewProductRecommendations::from_dto(dto.recommendations, repo).await?
        })
    }
}

impl NewSingleOwnedProduct {
    /// Returns the provider as a string.
    pub fn provider_as_string(&self) -> String {
        match &self.account_type {
            AccountType::IsaStocksAndShares(iss) => iss.provider.0.to_string(),
            AccountType::SelfInvestedPersonalPension(sipp) => sipp.provider.0.to_string(),
            AccountType::PersonalPension(pp) => pp.provider.0.to_string(),
            AccountType::JuniorIsaStocksAndShares(jisa) => jisa.provider.0.to_string(),
            AccountType::CashIsa(ci) => ci.provider.0.to_string(),
            AccountType::GeneralInvestmentAccount(gia) => gia.provider.0.to_string(),
            AccountType::OnshoreInvestmentBond(oib) => oib.provider.0.to_string(),
            AccountType::OffshoreInvestmentBond(oib) => oib.provider.0.to_string(),
        }
    }

    /// Returns the tax wrapper type as a string.
    pub fn tax_wrapper_type_as_string(&self) -> String {
        self.account_type.to_string()
    }

    /// Returns the tax wrapper type as a full name/brackets string.
    pub fn tax_wrapper_type_as_full_name_brackets_string_short_name(&self) -> String {
        self.account_type.account_type_as_full_name_brackets_string_short_name()
    }

    /// Returns the platform or account number.
    /// The platform or account number is a higher level account number within which numerous account or reference numbers can be 
    /// held which are unique identifiers to the individual tax wrappers themselves.
    pub fn platform_account_number(&self) -> &Option<PlatformAccountNumberType> {
        &self.platform_or_account_number
    }

    /// Returns the account or reference number as a string.
    /// The unique identifier for each tax wrapper held within a platform / wrap account.
    /// For new accounts where an account or reference number is not provided a temporary uuid is used in place.
    pub fn account_or_reference_number(&self) -> &Option<AccountOrReferenceNumberType> {
        &self.account_or_reference_number
    }

    /// Returns the account or reference number as a string.
    pub fn account_or_reference_number_or_id_as_string(&self) -> String {
        match &self.account_or_reference_number {
            Some(account_number) => account_number.to_string(),
            None => self.id.to_string()
        }
    }

    /// Returns a reference to the provider.
    pub fn provider(&self) -> &Provider {
        match &self.account_type {
            AccountType::IsaStocksAndShares(iss) => &iss.provider,
            AccountType::SelfInvestedPersonalPension(sipp) => &sipp.provider,
            AccountType::PersonalPension(pp) => &pp.provider,
            AccountType::JuniorIsaStocksAndShares(jisa) => &jisa.provider,
            AccountType::CashIsa(ci) => &ci.provider,
            AccountType::GeneralInvestmentAccount(gia) => &gia.provider,
            AccountType::OnshoreInvestmentBond(oib) => &oib.provider,
            AccountType::OffshoreInvestmentBond(oib) => &oib.provider,
        }
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CanBeJointlyOwnedAccountType {
    GeneralInvestmentAccount(GeneralInvestmentAccount),
    OnshoreInvestmentBond(OnshoreInvestmentBond),
    OffshoreInvestmentBond(OffshoreInvestmentBond),
}

impl CanBeJointlyOwnedAccountType {

    pub async fn from_dto<R>(
        dto: CanBeJointlyOwnedAccountTypeDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(
            match dto {
                CanBeJointlyOwnedAccountTypeDto::GeneralInvestmentAccount(inner_dto) => {
                    let inner = GeneralInvestmentAccount::from_dto(inner_dto, repo).await?;
                    CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(inner)
                } 
                CanBeJointlyOwnedAccountTypeDto::OffshoreInvestmentBond(inner_dto) => {
                    let inner = OffshoreInvestmentBond::from_dto(inner_dto, repo).await?;
                    CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(inner)
                } 
                CanBeJointlyOwnedAccountTypeDto::OnshoreInvestmentBond(inner_dto) => {
                    let inner = OnshoreInvestmentBond::from_dto(inner_dto, repo).await?;
                    CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(inner)
                } 
            }
        )
    }
}

// Implement Display for CanBeJointlyOwnedAccountTypes.
impl fmt::Display for CanBeJointlyOwnedAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(_) => "General Investment Account",
            CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(_) => "Onshore Investment Bond",
            CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(_) => "Offshore Investment Bond",
        };
        write!(f, "{}", type_str)
    }
}

impl CanBeJointlyOwnedAccountType {
    pub fn account_type_as_string_short_name(&self) -> String {
        match self {
            CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(_) => "GIA".to_string(),
            CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(_) => "Onshore Investment Bond".to_string(),
            CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(_) => "Offshore Investment Bond".to_string(),
        }
    }

    pub fn account_type_as_full_name_brackets_string_short_name(&self) -> String {
        match self {
            CanBeJointlyOwnedAccountType::GeneralInvestmentAccount(_) => "General Investment Account (GIA)".to_string(),
            CanBeJointlyOwnedAccountType::OnshoreInvestmentBond(_) => "Onshore Investment Bond".to_string(),
            CanBeJointlyOwnedAccountType::OffshoreInvestmentBond(_) => "Offshore Investment Bond".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AccountType {
    IsaStocksAndShares(IsaStocksAndShares),
    SelfInvestedPersonalPension(SelfInvestedPersonalPension),
    PersonalPension(PersonalPension),
    JuniorIsaStocksAndShares(JuniorIsaStocksAndShares),
    CashIsa(CashIsa),
    GeneralInvestmentAccount(GeneralInvestmentAccount),
    OnshoreInvestmentBond(OnshoreInvestmentBond),
    OffshoreInvestmentBond(OffshoreInvestmentBond)
}

// impl TryFrom<AccountTypeDto> for AccountType {
//     type Error = String;

//     fn try_from(dto: AccountTypeDto) -> Result<Self, Self::Error> {
//         match dto {
//             AccountTypeDto::IsaStocksAndShares(dto) => Ok(Self::IsaStocksAndShares(dto.try_into()?)),
//             AccountTypeDto::SelfInvestedPersonalPension(dto) => Ok(Self::SelfInvestedPersonalPension(dto.try_into()?)),
//             AccountTypeDto::PersonalPension(dto) => Ok(Self::PersonalPension(dto.try_into()?)),
//             AccountTypeDto::JuniorIsaStocksAndShares(dto) => Ok(Self::JuniorIsaStocksAndShares(dto.try_into()?)),
//             AccountTypeDto::CashIsa(dto) => Ok(Self::CashIsa(dto.try_into()?)),
//             AccountTypeDto::GeneralInvestmentAccount(dto) => Ok(Self::GeneralInvestmentAccount(dto.try_into()?)),
//             AccountTypeDto::OnshoreInvestmentBond(dto) => Ok(Self::OnshoreInvestmentBond(dto.try_into()?)),
//             AccountTypeDto::OffshoreInvestmentBond(dto) => Ok(Self::OffshoreInvestmentBond(dto.try_into()?)),
//         }
//     }
// }

impl AccountType {

    pub async fn from_dto<R>(
        dto: AccountTypeDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(
            match dto {
                AccountTypeDto::IsaStocksAndShares(inner_dto) => {
                    let inner = IsaStocksAndShares::from_dto(inner_dto, repo).await?;
                    AccountType::IsaStocksAndShares(inner)
                } 
                AccountTypeDto::SelfInvestedPersonalPension(inner_dto) => {
                    let inner = SelfInvestedPersonalPension::from_dto(inner_dto, repo).await?;
                    AccountType::SelfInvestedPersonalPension(inner)
                } 
                AccountTypeDto::PersonalPension(inner_dto) => {
                    let inner = PersonalPension::from_dto(inner_dto, repo).await?;
                    AccountType::PersonalPension(inner)
                } 
                AccountTypeDto::JuniorIsaStocksAndShares(inner_dto) => {
                    let inner = JuniorIsaStocksAndShares::from_dto(inner_dto, repo).await?;
                    AccountType::JuniorIsaStocksAndShares(inner)
                } 
                AccountTypeDto::CashIsa(inner_dto) => {
                    let inner = CashIsa::from_dto(inner_dto, repo).await?;
                    AccountType::CashIsa(inner)
                } 
                AccountTypeDto::GeneralInvestmentAccount(inner_dto) => {
                    let inner = GeneralInvestmentAccount::from_dto(inner_dto, repo).await?;
                    AccountType::GeneralInvestmentAccount(inner)
                } 
                AccountTypeDto::OffshoreInvestmentBond(inner_dto) => {
                    let inner = OffshoreInvestmentBond::from_dto(inner_dto, repo).await?;
                    AccountType::OffshoreInvestmentBond(inner)
                } 
                AccountTypeDto::OnshoreInvestmentBond(inner_dto) => {
                    let inner = OnshoreInvestmentBond::from_dto(inner_dto, repo).await?;
                    AccountType::OnshoreInvestmentBond(inner)
                } 
            }
        )
    }
}

// Implement Display for AccountType.
impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            AccountType::IsaStocksAndShares(_) => "ISA Stocks and Shares",
            AccountType::SelfInvestedPersonalPension(_) => "Self Invested Personal Pension",
            AccountType::PersonalPension(_) => "Personal Pension",
            AccountType::JuniorIsaStocksAndShares(_) => "Junior ISA Stocks and Shares",
            AccountType::CashIsa(_) => "Cash ISA",
            AccountType::GeneralInvestmentAccount(_) => "General Investment Account",
            AccountType::OffshoreInvestmentBond(_) => "Offshore Investment Bond",
            AccountType::OnshoreInvestmentBond(_) => "Onshore Investment Bond"
        };
        write!(f, "{}", type_str)
    }
}

impl AccountType {
    pub fn account_type_as_string_short_name(&self) -> String {
        match self {
            AccountType::IsaStocksAndShares(_) => "ISA".to_string(),
            AccountType::SelfInvestedPersonalPension(_) => "SIPP".to_string(),
            AccountType::PersonalPension(_) => "Personal Pension".to_string(),
            AccountType::JuniorIsaStocksAndShares(_) => "JISA".to_string(),
            AccountType::CashIsa(_) => "Cash ISA".to_string(),
            AccountType::GeneralInvestmentAccount(_) => "GIA".to_string(),
            AccountType::OffshoreInvestmentBond(_) => "Offshore Bond".to_string(),
            AccountType::OnshoreInvestmentBond(_) => "Onshore Bond".to_string()
        }
    }

    pub fn account_type_as_full_name_brackets_string_short_name(&self) -> String {
        match self {
            AccountType::IsaStocksAndShares(_) => "ISA Stocks and Shares (ISA)".to_string(),
            AccountType::SelfInvestedPersonalPension(_) => "Self Invested Personal Pension (SIPP)".to_string(),
            AccountType::PersonalPension(_) => "Personal Pension".to_string(),
            AccountType::JuniorIsaStocksAndShares(_) => "Junior ISA Stocks and Shares (JISA)".to_string(),
            AccountType::CashIsa(_) => "Cash ISA".to_string(),
            AccountType::GeneralInvestmentAccount(_) => "General Investment Account (GIA)".to_string(),
            AccountType::OffshoreInvestmentBond(_) => "Offshore Investment Bond".to_string(),
            AccountType::OnshoreInvestmentBond(_) => "Onshore Investment Bond".to_string()
        }
    }

    
}

/// Allow converting a string into an AccountType by matching the short names.
/// (Note: this requires that each inner type can be instantiated with Default.)
impl FromStr for AccountType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ISA" => Ok(AccountType::IsaStocksAndShares(Default::default())),
            "SIPP" => Ok(AccountType::SelfInvestedPersonalPension(Default::default())),
            "Personal Pension" => Ok(AccountType::PersonalPension(Default::default())),
            "JISA" => Ok(AccountType::JuniorIsaStocksAndShares(Default::default())),
            "Cash ISA" => Ok(AccountType::CashIsa(Default::default())),
            "GIA" => Ok(AccountType::GeneralInvestmentAccount(Default::default())),
            "Onshore Bond" => Ok(AccountType::OnshoreInvestmentBond(Default::default())),
            "Offshore Bond" => Ok(AccountType::OffshoreInvestmentBond(Default::default())),
            _ => Err(format!("Unknown account type: {}", s)),
        }
    }
}

/// Also allow TryFrom<String> for convenience.
impl TryFrom<String> for AccountType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<AccountType>()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsaStocksAndShares {
    provider: Provider,
    optional_description: Option<ConstrainedString200>,
    current_investment_strategy: InvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberType,
    charges: ProductCharges,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(IsaStocksAndSharesDto, InvestmentStrategy, ExistingProductRecommendations)> for IsaStocksAndShares {
    type Error = String;

    fn try_from((
        dto, 
        current_investment_strategy,
        existing_product_recommendations): (
            IsaStocksAndSharesDto, 
            InvestmentStrategy, 
            ExistingProductRecommendations
        )) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            current_investment_strategy: current_investment_strategy,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl IsaStocksAndShares {

    pub async fn from_dto<R>(
        dto: IsaStocksAndSharesDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.current_investment_strategy.clone(), repo)
            .await?;

        let existing_product_recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let sipp  = (
            dto,
            investment_strategy,
            existing_product_recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(sipp)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeneralInvestmentAccount {
    ownership: Ownership,
    provider: Provider,
    optional_description: Option<ConstrainedString200>,
    current_investment_strategy: InvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberType,
    charges: ProductCharges,
    current_tax_position: CapitalGainsPosition,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(GeneralInvestmentAccountDto, InvestmentStrategy, ExistingProductRecommendations)> for GeneralInvestmentAccount {
    type Error = String;

    fn try_from((
        dto, 
        current_investment_strategy,
        existing_product_recommendations): (
            GeneralInvestmentAccountDto, 
            InvestmentStrategy, 
            ExistingProductRecommendations
        )) -> Result<Self, Self::Error> {
        Ok(Self {
            ownership: dto.ownership.try_into()?,
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            current_investment_strategy: current_investment_strategy,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            current_tax_position: dto.current_tax_position.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl GeneralInvestmentAccount {

    pub async fn from_dto<R>(
        dto: GeneralInvestmentAccountDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.current_investment_strategy.clone(), repo)
            .await?;

        let existing_product_recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let gia  = (
            dto,
            investment_strategy,
            existing_product_recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(gia)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OnshoreInvestmentBond {
    ownership: Ownership,
    provider: Provider,
    optional_description: Option<ConstrainedString200>,
    current_investment_strategy: InvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberType,
    charges: ProductCharges,
    current_tax_position: ChargeableGainsPosition,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(OnshoreInvestmentBondDto, InvestmentStrategy, ExistingProductRecommendations)> for OnshoreInvestmentBond {
    type Error = String;

    fn try_from((
        dto, 
        current_investment_strategy,
        existing_product_recommendations): (
            OnshoreInvestmentBondDto, 
            InvestmentStrategy, 
            ExistingProductRecommendations
        )) -> Result<Self, Self::Error> {
        Ok(Self {
            ownership: dto.ownership.try_into()?,
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            current_investment_strategy: current_investment_strategy,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            current_tax_position: dto.current_tax_position.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl OnshoreInvestmentBond {

    pub async fn from_dto<R>(
        dto: OnshoreInvestmentBondDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.current_investment_strategy.clone(), repo)
            .await?;

        let existing_product_recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let onb  = (
            dto,
            investment_strategy,
            existing_product_recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(onb)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OffshoreInvestmentBond {
    ownership: Ownership,
    provider: Provider,
    optional_description: Option<ConstrainedString200>,
    current_investment_strategy: InvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberType,
    charges: ProductCharges,
    current_tax_position: ChargeableGainsPosition,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(OffshoreInvestmentBondDto, InvestmentStrategy, ExistingProductRecommendations)> for OffshoreInvestmentBond {
    type Error = String;

    fn try_from((
        dto, 
        current_investment_strategy,
        existing_product_recommendations): (
            OffshoreInvestmentBondDto, 
            InvestmentStrategy, 
            ExistingProductRecommendations
        )) -> Result<Self, Self::Error> {
        Ok(Self {
            ownership: dto.ownership.try_into()?,
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            current_investment_strategy: current_investment_strategy,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            current_tax_position: dto.current_tax_position.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl OffshoreInvestmentBond {

    pub async fn from_dto<R>(
        dto: OffshoreInvestmentBondDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.current_investment_strategy.clone(), repo)
            .await?;

        let existing_product_recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let offb  = (
            dto,
            investment_strategy,
            existing_product_recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(offb)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfInvestedPersonalPension {
    provider: Provider,
    optional_description: Option<ConstrainedString200>,
    current_investment_strategy: InvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberType,
    charges: ProductCharges,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(SelfInvestedPersonalPensionDto, InvestmentStrategy, ExistingProductRecommendations)> for SelfInvestedPersonalPension {
    type Error = String;

    fn try_from((
        dto, 
        current_investment_strategy,
        existing_product_recommendations): (
            SelfInvestedPersonalPensionDto, 
            InvestmentStrategy, 
            ExistingProductRecommendations
        )) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            current_investment_strategy: current_investment_strategy,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl SelfInvestedPersonalPension {

    pub async fn from_dto<R>(
        dto: SelfInvestedPersonalPensionDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.current_investment_strategy.clone(), repo)
            .await?;

        let existing_product_recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let sipp  = (
            dto,
            investment_strategy,
            existing_product_recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(sipp)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPension {
    provider: Provider,
    optional_description: Option<ConstrainedString200>,
    current_investment_strategy: InvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberType,
    charges: ProductCharges,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(PersonalPensionDto, InvestmentStrategy, ExistingProductRecommendations)> for PersonalPension {
    type Error = String;

    fn try_from((
        dto, 
        current_investment_strategy,
        existing_product_recommendations): (
            PersonalPensionDto, 
            InvestmentStrategy, 
            ExistingProductRecommendations
        )) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            current_investment_strategy: current_investment_strategy,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl PersonalPension {

    pub async fn from_dto<R>(
        dto: PersonalPensionDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.current_investment_strategy.clone(), repo)
            .await?;

        let existing_product_recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let pp  = (
            dto,
            investment_strategy,
            existing_product_recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(pp)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct JuniorIsaStocksAndShares {
    provider: Provider,
    optional_description: Option<ConstrainedString200>,
    current_investment_strategy: InvestmentStrategy,
    current_value: Valuation,
    linked_cash_or_fee_payment_wrapper: AccountOrReferenceNumberType,
    charges: ProductCharges,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(JuniorIsaStocksAndSharesDto, InvestmentStrategy, ExistingProductRecommendations)> for JuniorIsaStocksAndShares {
    type Error = String;

    fn try_from((
        dto, 
        current_investment_strategy,
        existing_product_recommendations): (
            JuniorIsaStocksAndSharesDto, 
            InvestmentStrategy, 
            ExistingProductRecommendations
        )) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            current_investment_strategy: current_investment_strategy,
            current_value: dto.current_value.try_into()?,
            linked_cash_or_fee_payment_wrapper: dto.linked_cash_or_fee_payment_wrapper.try_into()?,
            charges: dto.charges.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl JuniorIsaStocksAndShares {

    pub async fn from_dto<R>(
        dto: JuniorIsaStocksAndSharesDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.current_investment_strategy.clone(), repo)
            .await?;

        let existing_product_recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let jisa  = (
            dto,
            investment_strategy,
            existing_product_recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(jisa)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CashIsa {
    provider: Provider,
    account_number: BankAccountNumber,
    sort_code: BankSortCode,
    optional_description: Option<ConstrainedString200>,
    current_value: Valuation,
    recommendations: ExistingProductRecommendations,
}

impl TryFrom<(CashIsaDto, ExistingProductRecommendations)> for CashIsa {
    type Error = String;

    fn try_from((dto, existing_product_recommendations): (CashIsaDto, ExistingProductRecommendations)) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: dto.provider.try_into()?,
            optional_description: dto.optional_description.map(|dto| dto.try_into()).transpose()?,
            account_number: dto.account_number.try_into()?,
            sort_code: dto.sort_code.try_into()?,
            current_value: dto.current_value.try_into()?,
            recommendations: existing_product_recommendations
        })
    }
}

impl CashIsa {

    pub async fn from_dto<R>(
        dto: CashIsaDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync
    {
        let recommendations = ExistingProductRecommendations::from_dto(dto.recommendations.clone(), repo)
            .await?;

        let jisa  = (
            dto,
            recommendations
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(jisa)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
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
            ProvidersDto::Fidelity => Ok(Provider(Providers::Fidelity)),
            ProvidersDto::JamesHay => Ok(Provider(Providers::JamesHay)),
        }
    }
}

impl Provider {
    pub fn value(&self) -> &Providers {
        &self.0
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Providers {
    Abrdn,
    #[default]
    Transact,
    Utmost,
    ReAssure,
    Quilter,
    Fidelity,
    JamesHay
}

impl fmt::Display for Providers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let provider_str = match self {
            Providers::Abrdn => "abrdn",  // lowercase 'a' for abrdn
            Providers::Transact => "Transact",
            Providers::Utmost => "Utmost",
            Providers::ReAssure => "ReAssure",
            Providers::Quilter => "Quilter",
            Providers::Fidelity => "Fidelity",
            Providers::JamesHay => "James Hay"
        };
        write!(f, "{}", provider_str)
    }
}

impl Providers {
    /// Returns an alternative name for the provider.
    pub fn alt_name(&self) -> &str {
        match self {
            Providers::Abrdn => "abrdn Wrap", // Special case for abrdn
            Providers::Transact => "Transact",
            Providers::Utmost => "Utmost",
            Providers::ReAssure => "ReAssure",
            Providers::Quilter => "Quilter",
            Providers::Fidelity => "Fidelity",
            Providers::JamesHay => "James Hay"
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductCharges {
    ongoing_advice_charge: Percentage,
    platform_charge: Percentage,
    ongoing_fund_charge: Option<Percentage>,
    other_charges: Option<OtherCharge>
}

impl TryFrom<ProductChargesDto> for ProductCharges {
    type Error = String;

    fn try_from(dto: ProductChargesDto) -> Result<Self, Self::Error> {
        Ok(Self {
            ongoing_advice_charge: dto.ongoing_advice_charge.try_into()?,
            platform_charge: dto.platform_charge.try_into()?,
            ongoing_fund_charge: dto.ongoing_fund_charge.map(|dto| dto.try_into()).transpose()?,
            other_charges: dto.other_charges.map(|dto|dto.try_into()).transpose()?
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExistingProductRecommendations {
    product_retention: ProductRetention,
}

impl ExistingProductRecommendations {

    pub async fn from_dto<R>(
        dto: ExistingProductRecommendationsDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(Self {
            product_retention: ProductRetention::from_dto(dto.product_retention, repo).await?,
        })
    }

}



#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewProductRecommendations {
    rationale: ConstrainedString1000,
    recommended_product_charges: ProductCharges,
    recommended_investment_strategy: InvestmentStrategy,
    linked_objectives: Vec<Uuid>,
    recommendation_actions: Vec<RecommendedAction>
}

impl TryFrom<(NewProductRecommendationsDto, InvestmentStrategy)> for NewProductRecommendations {
    type Error = String;

    fn try_from((dto, investment_strategy): (NewProductRecommendationsDto, InvestmentStrategy)) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?,
            recommended_product_charges: dto.recommended_product_charges.try_into()?,
            recommended_investment_strategy: investment_strategy,
            linked_objectives: dto.linked_objectives,
            recommendation_actions: dto.recommendation_actions.iter().map(|dto| dto.clone().try_into()).collect::<Result<_, _>>()?
        })
    }
}

impl NewProductRecommendations {

    pub async fn from_dto<R>(
        dto: NewProductRecommendationsDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio>
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.recommended_investment_strategy.clone(), repo)
            .await?;

        let iss  = (
            dto,
            investment_strategy
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(iss)

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

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct RecommendedInvestmentAndRiskStrategy {
//     recommended_investment_strategy: InvestmentStrategy,
//     risk_level: RiskProfile
// }

// impl TryFrom<RecommendedInvestmentAndRiskStrategyDto> for RecommendedInvestmentAndRiskStrategy {
//     type Error = String;

//     fn try_from(dto: RecommendedInvestmentAndRiskStrategyDto) -> Result<Self, Self::Error> {
//         Ok(Self {
//             recommended_investment_strategy: dto.recommended_investment_strategy.try_into()?,
//             risk_level: dto.risk_level.try_into()?
//         })
//     }
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum CapitalGainsPosition {
    CapitalGainsTaxAvoidLiability(CapitalGainsTaxAvoidLiability),
    CapitalGainsTaxNoLiability(CapitalGainsTaxNoLiability),
    CapitalGainsTaxIncurLiability(CapitalGainsTaxIncurLiability)
}

impl Default for CapitalGainsPosition {
    fn default() -> Self {
        Self::CapitalGainsTaxNoLiability(
            CapitalGainsTaxNoLiability{ 
                unrealised_gains: ConstrainedMoneyAmountMedium::default(),
                capital_gains_tax_discussion: ConstrainedString1000::default()
            }
        )
    }
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

impl Default for ChargeableGainsPosition {
    fn default() -> Self {
        Self::ChargeableGainsTaxNoLiability(
            ChargeableGainsTaxNoLiability{ 
                unrealised_gains: ConstrainedMoneyAmountMedium::default(),
                chargeable_gains_tax_discussion: ConstrainedString1000::default()
            }
        )
    }
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
}

impl Default for ProductRetention {
    fn default() -> Self {
        Self::FullyEncash(FullyEncash { rationale: ConstrainedString1000::default() })
    }
}

impl ProductRetention {

    pub async fn from_dto<R>(
        dto: ProductRetentionDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(
            match dto {
                ProductRetentionDto::Retain(inner_dto) => {
                    let inner = Retain::from_dto(inner_dto, repo).await?;
                    ProductRetention::Retain(inner)
                } 
                ProductRetentionDto::Replace(inner_dto) => Self::Replace(inner_dto.try_into()?),
                
                ProductRetentionDto::FullyEncash(inner_dto) => Self::FullyEncash(inner_dto.try_into()?),
                
            }
        )
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
            RecommendedActionDto::Transfer(transfer_dto) => Ok(Self::Transfer(transfer_dto.try_into()?))
        }
    }
}

impl RecommendedAction {
    /// Returns a descriptive string for the recommended action.
    pub fn description(&self) -> &'static str {
        match self {
            RecommendedAction::SingleWithdrawal(_) => "Single Withdrawal",
            RecommendedAction::SingleContribution(_) => "Single Contribution",
            RecommendedAction::RegularContribution(_) => "Regular Contribution",
            RecommendedAction::RegularWithdrawal(_) => "Regular Withdrawal",
            RecommendedAction::Transfer(_) => "Transfer",
            RecommendedAction::StopWithdrawal(_) => "Stop Withdrawal",
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
    recommended_product_charges: ProductCharges,
    recommended_investment_strategy: RealignOrRebalance,
    linked_objectives: Vec<Uuid>,
    recommendation_actions: Option<Vec<RecommendedAction>>
}

impl Retain {

    pub async fn from_dto<R>(
        dto: RetainDto,
        repo: &R,
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        Ok(Self {
            rationale: dto.rationale.try_into()?,
            recommended_product_charges: dto.recommended_product_charges.try_into()?,
            recommended_investment_strategy: RealignOrRebalance::from_dto(dto.recommended_investment_strategy, repo).await?,
            linked_objectives: dto
                .linked_objectives
                .iter()
                .map(|uuid_str| Uuid::parse_str(uuid_str.as_str()))
                .collect::<Result<Vec<_>, _>>() 
                .map_err(|e| e.to_string())?,
            recommendation_actions: dto
                .recommendation_actions
                .map(|actions| {
                    actions
                        .into_iter()
                        .map(|action_dto| action_dto.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl Retain {

    pub fn recommendation_actions(&self) -> &Option<Vec<RecommendedAction>> {
        &self.recommendation_actions
    }

    /// Returns a HashMap grouping the recommended actions by their type.
    ///
    /// The key is a string (derived from the `description()` method of `RecommendedAction`),
    /// and the value is a vector of recommended actions with that description.
    pub fn actions_by_action_type(&self) -> HashMap<String, Vec<RecommendedAction>> {
        let mut groups: HashMap<String, Vec<RecommendedAction>> = HashMap::new();
        if let Some(actions) = &self.recommendation_actions {
            for action in actions.iter() {
                let key = action.description().to_string();
                groups.entry(key).or_insert_with(Vec::new).push(action.clone());
            }
        }
        groups
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RealignOrRebalance {
    Realign(Realign),
    Rebalance(Rebalance)
}

impl RealignOrRebalance {

    pub async fn from_dto<R>(
        dto: RealignOrRebalanceDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio>
    {

        Ok(
            match dto {
                RealignOrRebalanceDto::Realign(inner_dto) => {
                    let inner = Realign::from_dto(inner_dto, repo).await?;
                    RealignOrRebalance::Realign(inner)
                }
                RealignOrRebalanceDto::Rebalance(inner_dto) => {
                    let inner = Rebalance::from_dto(inner_dto, repo).await?;
                    RealignOrRebalance::Rebalance(inner)
                }
            }
        )

    }

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Realign {
    rationale: ConstrainedString1000,
    recommended_investment_strategy: InvestmentStrategy
}

impl TryFrom<(RealignDto, InvestmentStrategy)> for Realign {
    type Error = String;

    fn try_from((dto, investment_strategy): (RealignDto, InvestmentStrategy)) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?,
            recommended_investment_strategy: investment_strategy,
        })
    }
}

impl Realign {

    pub async fn from_dto<R>(
        dto: RealignDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio>
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.recommended_investment_strategy.clone(), repo)
            .await?;

        let rebalance  = (
            dto,
            investment_strategy
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(rebalance)

    }

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rebalance {
    rationale: ConstrainedString1000,
    recommended_investment_strategy: InvestmentStrategy
}

impl TryFrom<(RebalanceDto, InvestmentStrategy)> for Rebalance {
    type Error = String;

    fn try_from((dto, investment_strategy): (RebalanceDto, InvestmentStrategy)) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?,
            recommended_investment_strategy: investment_strategy,
        })
    }
}

impl Rebalance {

    pub async fn from_dto<R>(
        dto: RebalanceDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio>
    {
        let investment_strategy = InvestmentStrategy::from_dto(dto.recommended_investment_strategy.clone(), repo)
            .await?;

        let rebalance  = (
            dto,
            investment_strategy
        )
            .try_into()
            .map_err(|e| e)?;

        Ok(rebalance)

    }

}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Replace {
    FullyReplace(FullyReplace),
    PartiallyReplace(PartiallyReplace)
}

impl TryFrom<ReplaceDto> for Replace {
    type Error = String;

    fn try_from(dto: ReplaceDto) -> Result<Self, Self::Error> {
        match dto {
            ReplaceDto::FullyReplace(fully_replace_dto) => {
                Ok(Replace::FullyReplace(fully_replace_dto.try_into()?))
            }
            ReplaceDto::PartiallyReplace(partially_replace_dto) => {
                Ok(Replace::PartiallyReplace(partially_replace_dto.try_into()?))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullyReplace {
    rationale: ConstrainedString1000,
    replacement_product_information: ReplacementProductInformation,
    replace_to_details: FullyReplaceDetail,
    linked_objectives: Vec<Uuid>
}

impl FullyReplace {
    pub fn rationale(&self) -> &ConstrainedString1000 {
        &self.rationale
    }

    pub fn replacement_product_information(&self) -> &ReplacementProductInformation {
        &self.replacement_product_information
    }

    pub fn replace_to_details(&self) -> &FullyReplaceDetail {
        &self.replace_to_details
    }

    pub fn linked_objectives(&self) -> &Vec<Uuid> {
        &self.linked_objectives
    }
}

impl TryFrom<FullyReplaceDto> for FullyReplace {
    type Error = String;

    fn try_from(dto: FullyReplaceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?,
            replacement_product_information: dto.replacement_product_information.try_into()?,
            replace_to_details: dto.replace_to_details.try_into()?,
            linked_objectives: dto.linked_objectives.iter().map(|dto|Uuid::parse_str(&dto.as_str())).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullyReplaceDetail {
    method_of_transfer: Vec<MethodOfTransfer>,
    transfer_to_account_or_reference_number: AccountOrReferenceNumberType
}

impl FullyReplaceDetail {
    /// Returns a reference to the method of transfer.
    pub fn method_of_transfer(&self) -> &Vec<MethodOfTransfer> {
        &self.method_of_transfer
    }

    /// Returns a reference to the transfer-to account or reference number.
    pub fn transfer_to_account_or_reference_number(&self) -> &AccountOrReferenceNumberType {
        &self.transfer_to_account_or_reference_number
    }
}


impl TryFrom<FullyReplaceDetailDto> for FullyReplaceDetail {
    type Error = String;

    fn try_from(dto: FullyReplaceDetailDto) -> Result<Self, Self::Error> {
        Ok(Self {
            method_of_transfer: dto.method_of_transfer
                .iter()
                .map(|dto| dto.clone().try_into()) 
                .collect::<Result<Vec<_>, _>>()?,

            transfer_to_account_or_reference_number: dto.transfer_to_account_or_reference_number.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MethodOfTransfer {
    InSpecieMethod(InSpecieMethod),
    CashMethod(CashMethod)
}

impl TryFrom<MethodOfTransferDto> for MethodOfTransfer {
    type Error = String;

    fn try_from(dto: MethodOfTransferDto) -> Result<Self, Self::Error> {
        match dto {
            MethodOfTransferDto::CashMethod(dto) => Ok(MethodOfTransfer::CashMethod(dto.try_into()?)),
            MethodOfTransferDto::InSpecieMethod(dto) => Ok(MethodOfTransfer::InSpecieMethod(dto.try_into()?)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InSpecieMethod {
    value: ConstrainedMoneyAmountLarge,
    funds_to_inspecie_transfer: Vec<FundHolding>
}

impl InSpecieMethod {
    /// Returns the total value of the in-specie transfer.
    pub fn value(&self) -> &ConstrainedMoneyAmountLarge {
        &self.value
    }

    /// Returns a reference to the funds to be transferred in-specie.
    pub fn funds_to_inspecie_transfer(&self) -> &Vec<FundHolding> {
        &self.funds_to_inspecie_transfer
    }
}


impl TryFrom<InSpecieMethodDto> for InSpecieMethod {
    type Error = String;

    fn try_from(dto: InSpecieMethodDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            funds_to_inspecie_transfer: dto
                    .funds_to_inspecie_transfer
                    .iter()
                    .map(|dto| dto.clone().try_into()) 
                    .collect::<Result<Vec<_>, _>>()?

        })
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CashMethod {
    value: ConstrainedMoneyAmountLarge
}

impl CashMethod {
    /// Returns the total cash value for the transfer.
    pub fn value(&self) -> &ConstrainedMoneyAmountLarge {
        &self.value
    }
}


impl TryFrom<CashMethodDto> for CashMethod {
    type Error = String;

    fn try_from(dto: CashMethodDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartiallyReplace {
    rationale: ConstrainedString1000,
    replacement_product_information: ReplacementProductInformation,
    partially_replace_to_details: PartiallyReplaceDetail,
    linked_objectives: Vec<Uuid>
}

impl PartiallyReplace {
    /// Returns the rationale for partial replacement.
    pub fn rationale(&self) -> &ConstrainedString1000 {
        &self.rationale
    }

    /// Returns a reference to the replacement product information.
    pub fn replacement_product_information(&self) -> &ReplacementProductInformation {
        &self.replacement_product_information
    }

    /// Returns a reference to the partial replacement details.
    pub fn partially_replace_to_details(&self) -> &PartiallyReplaceDetail {
        &self.partially_replace_to_details
    }

    /// Returns a reference to the linked objectives.
    pub fn linked_objectives(&self) -> &Vec<Uuid> {
        &self.linked_objectives
    }
}


impl TryFrom<PartiallyReplaceDto> for PartiallyReplace {
    type Error = String;

    fn try_from(dto: PartiallyReplaceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            rationale: dto.rationale.try_into()?, 
            replacement_product_information: dto.replacement_product_information.try_into()?, 
            partially_replace_to_details: dto.partially_replace_to_details.try_into()?, 
            linked_objectives: dto
                .linked_objectives
                .iter()
                .map(|uuid_str| Uuid::parse_str(uuid_str.as_str()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?, 
        })
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartiallyReplaceDetail {
    method_of_transfer: Vec<MethodOfTransfer>,
    transfer_to_account_or_reference_number: AccountOrReferenceNumberType,
    amount_to_be_left_in_existing_product: ConstrainedMoneyAmountLarge,
    reason_for_leaving_in_existing_product: ConstrainedString1000
}

impl PartiallyReplaceDetail {
    /// Returns a reference to the method of transfer.
    pub fn method_of_transfer(&self) -> &Vec<MethodOfTransfer> {
        &self.method_of_transfer
    }

    /// Returns a reference to the transfer-to account or reference number.
    pub fn transfer_to_account_or_reference_number(&self) -> &AccountOrReferenceNumberType {
        &self.transfer_to_account_or_reference_number
    }

    /// Returns the amount that will be left in the existing product.
    pub fn amount_to_be_left_in_existing_product(&self) -> &ConstrainedMoneyAmountLarge {
        &self.amount_to_be_left_in_existing_product
    }

    /// Returns the reason for leaving funds in the existing product.
    pub fn reason_for_leaving_in_existing_product(&self) -> &ConstrainedString1000 {
        &self.reason_for_leaving_in_existing_product
    }
}


impl TryFrom<PartiallyReplaceDetailDto> for PartiallyReplaceDetail {
    type Error = String;

    fn try_from(dto: PartiallyReplaceDetailDto) -> Result<Self, Self::Error> {
        Ok(Self {
            method_of_transfer: dto.method_of_transfer
                .iter()
                .map(|dto| dto.clone().try_into())
                .collect::<Result<Vec<_>, _>>()?,

            transfer_to_account_or_reference_number: dto.transfer_to_account_or_reference_number.try_into()?,
            amount_to_be_left_in_existing_product: dto.amount_to_be_left_in_existing_product.try_into()?,
            reason_for_leaving_in_existing_product: dto.reason_for_leaving_in_existing_product.try_into()?,
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
    rationale: Option<ConstrainedString1000>,
    date_of_action: Option<Date>,
    tax_year_of_action: Option<TaxYear>
}

impl TryFrom<SingleContributionDto> for SingleContribution {
    type Error = String;

    fn try_from(dto: SingleContributionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description: dto.executive_summary_description.try_into()?,
            rationale: dto.rationale.map(|dto|dto.try_into()).transpose()?,
            date_of_action: if dto.date_of_action.is_some() { Some(dto.date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None }
        })
    }
}

impl SingleContribution {
    /// Returns a reference to the contribution value.
    pub fn value(&self) -> &ConstrainedMoneyAmountLarge {
        &self.value
    }

    /// Returns a reference to the executive summary description.
    pub fn executive_summary_description(&self) -> &ConstrainedString200 {
        &self.executive_summary_description
    }

    /// Returns an optional reference to the rationale.
    pub fn rationale(&self) -> Option<&ConstrainedString1000> {
        self.rationale.as_ref()
    }

    /// Returns an optional reference to the date of action.
    pub fn date_of_action(&self) -> Option<&Date> {
        self.date_of_action.as_ref()
    }

    /// Returns an optional reference to the tax year of action.
    pub fn tax_year_of_action(&self) -> Option<&TaxYear> {
        self.tax_year_of_action.as_ref()
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
    executive_summary_description_receiving_product: Option<ConstrainedString200>,
    executive_summary_description_transferring_product: Option<ConstrainedString200>,
    rationale: ConstrainedString1000,
    date_of_action: Option<Date>,
    tax_year_of_action: Option<TaxYear>,
    transfer_to_details: TransferDetail
}

impl TryFrom<TransferDto> for Transfer {
    type Error = String;

    fn try_from(dto: TransferDto) -> Result<Self, Self::Error> {

        Ok(Self {
            value: dto.value.try_into()?,
            executive_summary_description_receiving_product: dto.executive_summary_description_receiving_product.map(|dto| dto.try_into()).transpose()?,
            executive_summary_description_transferring_product: dto. executive_summary_description_transferring_product.map(|dto| dto.try_into()).transpose()?,
            rationale: dto.rationale.try_into()?,
            date_of_action: if dto.date_of_action.is_some() { Some(dto.date_of_action.unwrap().try_into()?) } else { None },
            tax_year_of_action: if dto.tax_year_of_action.is_some() { Some(dto.tax_year_of_action.unwrap().try_into()?) } else { None },
            transfer_to_details: dto.transfer_details.try_into()?
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
pub struct TransferDetail {
    pub transfer_to_account_or_reference_number: AccountOrReferenceNumberType,
    pub transfer_from_account_or_reference_number: KnownOrUnknownAccount,
}

impl TryFrom<TransferDetailDto> for TransferDetail {
    type Error = String;

    fn try_from(dto: TransferDetailDto) -> Result<Self, Self::Error> {
        Ok(Self {
            transfer_to_account_or_reference_number: dto.transfer_to_account_or_reference_number.try_into()?,
            transfer_from_account_or_reference_number: dto.transfer_from_account_or_reference_number.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum KnownOrUnknownAccount {
    Known(AccountOrReferenceNumberType),
    Unknown { description: ConstrainedString200, account_type: AccountType }
}

impl TryFrom<KnownOrUnknownAccountDto> for KnownOrUnknownAccount {
    type Error = String;

    fn try_from(dto: KnownOrUnknownAccountDto) -> Result<Self, Self::Error> {
        match dto {
            KnownOrUnknownAccountDto::Known(account_or_reference_number_type) => Ok(KnownOrUnknownAccount::Known(account_or_reference_number_type.try_into()?)),
            KnownOrUnknownAccountDto::Unknown{ description, account_type } => { 
                Ok(KnownOrUnknownAccount::Unknown{ 
                        description: description.try_into()?, 
                        account_type: account_type.try_into()?
                }) 
            }
        }
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
#[serde(tag = "type", content = "content")]
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
#[serde(tag = "type", content = "content")]
pub enum AccountOrReferenceNumberType {
    Abrdn(AbrdnFullAccountNumber),
    AbrdnSipp(AbrdnSippNumber),
    Transact(TransactReferenceNumber),
    Other(ConstrainedString200),
    NewAccount(Uuid)
}

impl Default for AccountOrReferenceNumberType {
    fn default() -> Self {
        Self::Other(ConstrainedString200::default())
    }
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