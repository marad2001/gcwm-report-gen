use std::fmt;

use serde::{Deserialize, Serialize};

use super::risk_assessment_dto::RiskProfileDto;

/// The “model” metadata that identifies *which* GCWM portfolio you mean:
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModelPortfolioIdDto {
    pub provider: InvestmentStrategyProviderDto,       
    pub service_proposition: InvestmentStrategyServicePropositionDto,         
    pub sri: bool,                
    pub risk_profile: RiskProfileDto,     
    pub product_type: InvestmentStrategyProductTypeDto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "investmentStrategyProvider")]
pub enum InvestmentStrategyProviderDto {
    Transact,
    Abrdn,
}

impl fmt::Display for InvestmentStrategyProviderDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            InvestmentStrategyProviderDto::Transact => "Transact",
            InvestmentStrategyProviderDto::Abrdn    => "Abrdn",
        };
        write!(f, "{}", s)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "investmentStrategyProposition")]
pub enum InvestmentStrategyServicePropositionDto {
    Prime,
    Active,
}

impl fmt::Display for InvestmentStrategyServicePropositionDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            InvestmentStrategyServicePropositionDto::Prime  => "Prime",
            InvestmentStrategyServicePropositionDto::Active => "Active",
        };
        write!(f, "{}", s)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "investmentStrategyProductType")]
pub enum InvestmentStrategyProductTypeDto {
    Standard,
    Sipp,
}

impl fmt::Display for InvestmentStrategyProductTypeDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            InvestmentStrategyProductTypeDto::Standard => "Standard",
            InvestmentStrategyProductTypeDto::Sipp     => "Sipp",
        };
        write!(f, "{}", s)
    }
}

/// A time‐stamped portfolio (the actual holdings & charges):
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionedPortfolioDto {
    pub id: ModelPortfolioIdDto,
    pub effective_date: MonthYearDto, // e.g. Aug2024
    pub fund_charges: Option<f32>      
}

/// Bespoke portfolios don’t have an “id,” only the holdings & when they were created:
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BespokePortfolioDto {
    pub created: MonthYearDto,
    pub portfolio: BespokeInvestmentPortfolioDto,
    pub fund_charges: Option<f32>,
    pub risk_level: RiskProfileDto,  
}

/// Finally, an investment strategy is just *one* of those two:
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum InvestmentStrategyDto {
  /// A GCWM model portfolio, possibly past (if `as_of < today`) or still current.
  Model(VersionedPortfolioDto),

  /// A bespoke portfolio, created at a point in time.
  Bespoke(BespokePortfolioDto),
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentPortfolioDto {
    pub fund_holdings: Vec<FundHoldingDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BespokeInvestmentPortfolioDto {
    pub fund_holdings: Option<Vec<FundHoldingDto>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MonthYearDto {
    Aug24
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundHoldingDto {
    pub fund_name: String,
    pub isin: Option<String>,
    pub sedol: Option<String>,
    pub value: Option<f64>,
    pub percentage_of_portfolio: Option<f32>,
    pub fund_charge: f32
}

























// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum InvestmentStrategyDto {
//     PastInvestmentStrategy(PastInvestmentStrategyDto),
//     InvestableInvestmentStrategy(InvestableInvestmentStrategyDto)
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum PastInvestmentStrategyDto {
//     BespokeInvestmentStrategy(InvestmentPortfolioDto),
//     GCWMInvestmentStrategy(GCWMPastInvestmentStrategyDto)
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum InvestableInvestmentStrategyDto {
//     BespokeInvestmentStrategy(InvestmentPortfolioDto),
//     GCWMInvestmentStrategy(GCWMPresentInvestmentStrategyDto)
// }



// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum GCWMPastInvestmentStrategyDto {
//     TransactPrimeCautious(StrategyMonthYearDto),
//     TransactPrimeCautiousToModerate(StrategyMonthYearDto),
//     TransactPrimeModerate(StrategyMonthYearDto),
//     TransactPrimeModerateToAdventurous(StrategyMonthYearDto),
//     TransactPrimeAdventurous(StrategyMonthYearDto),
//     TransactPrimeSriCautious(StrategyMonthYearDto),
//     TransactPrimeSriCautiousToModerate(StrategyMonthYearDto),
//     TransactPrimeSriModerate(StrategyMonthYearDto),
//     TransactPrimeSriModerateToAdventurous(StrategyMonthYearDto),
//     TransactPrimeSriAdventurous(StrategyMonthYearDto),
//     TransactActiveCautious(StrategyMonthYearDto),
//     TransactActiveCautiousToModerate(StrategyMonthYearDto),
//     TransactActiveModerate(StrategyMonthYearDto),
//     TransactActiveModerateToAdventurous(StrategyMonthYearDto),
//     TransactActiveAdventurous(StrategyMonthYearDto),
//     TransactActiveSriCautious(StrategyMonthYearDto),
//     TransactActiveSriCautiousToModerate(StrategyMonthYearDto),
//     TransactActiveSriModerate(StrategyMonthYearDto),
//     TransactActiveSriModerateToAdventurous(StrategyMonthYearDto),
//     TransactActiveSriAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeCautious(StrategyMonthYearDto),
//     AbrdnPrimeCautiousToModerate(StrategyMonthYearDto),
//     AbrdnPrimeModerate(StrategyMonthYearDto),
//     AbrdnPrimeModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeSriCautious(StrategyMonthYearDto),
//     AbrdnPrimeSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnPrimeSriModerate(StrategyMonthYearDto),
//     AbrdnPrimeSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeSriAdventurous(StrategyMonthYearDto),
//     AbrdnActiveCautious(StrategyMonthYearDto),
//     AbrdnActiveCautiousToModerate(StrategyMonthYearDto),
//     AbrdnActiveModerate(StrategyMonthYearDto),
//     AbrdnActiveModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnActiveAdventurous(StrategyMonthYearDto),
//     AbrdnActiveSriCautious(StrategyMonthYearDto),
//     AbrdnActiveSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnActiveSriModerate(StrategyMonthYearDto),
//     AbrdnActiveSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnActiveSriAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeCautious(StrategyMonthYearDto),
//     AbrdnSippPrimeCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeSriCautious(StrategyMonthYearDto),
//     AbrdnSippPrimeSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeSriModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeSriAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveCautious(StrategyMonthYearDto),
//     AbrdnSippActiveCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippActiveModerate(StrategyMonthYearDto),
//     AbrdnSippActiveModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveSriCautious(StrategyMonthYearDto),
//     AbrdnSippActiveSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippActiveSriModerate(StrategyMonthYearDto),
//     AbrdnSippActiveSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveSriAdventurous(StrategyMonthYearDto),
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum GCWMPresentInvestmentStrategyDto {
//     TransactPrimeCautious(StrategyMonthYearDto),
//     TransactPrimeCautiousToModerate(StrategyMonthYearDto),
//     TransactPrimeModerate(StrategyMonthYearDto),
//     TransactPrimeModerateToAdventurous(StrategyMonthYearDto),
//     TransactPrimeAdventurous(StrategyMonthYearDto),
//     TransactPrimeSriCautious(StrategyMonthYearDto),
//     TransactPrimeSriCautiousToModerate(StrategyMonthYearDto),
//     TransactPrimeSriModerate(StrategyMonthYearDto),
//     TransactPrimeSriModerateToAdventurous(StrategyMonthYearDto),
//     TransactPrimeSriAdventurous(StrategyMonthYearDto),
//     TransactActiveCautious(StrategyMonthYearDto),
//     TransactActiveCautiousToModerate(StrategyMonthYearDto),
//     TransactActiveModerate(StrategyMonthYearDto),
//     TransactActiveModerateToAdventurous(StrategyMonthYearDto),
//     TransactActiveAdventurous(StrategyMonthYearDto),
//     TransactActiveSriCautious(StrategyMonthYearDto),
//     TransactActiveSriCautiousToModerate(StrategyMonthYearDto),
//     TransactActiveSriModerate(StrategyMonthYearDto),
//     TransactActiveSriModerateToAdventurous(StrategyMonthYearDto),
//     TransactActiveSriAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeCautious(StrategyMonthYearDto),
//     AbrdnPrimeCautiousToModerate(StrategyMonthYearDto),
//     AbrdnPrimeModerate(StrategyMonthYearDto),
//     AbrdnPrimeModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeSriCautious(StrategyMonthYearDto),
//     AbrdnPrimeSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnPrimeSriModerate(StrategyMonthYearDto),
//     AbrdnPrimeSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnPrimeSriAdventurous(StrategyMonthYearDto),
//     AbrdnActiveCautious(StrategyMonthYearDto),
//     AbrdnActiveCautiousToModerate(StrategyMonthYearDto),
//     AbrdnActiveModerate(StrategyMonthYearDto),
//     AbrdnActiveModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnActiveAdventurous(StrategyMonthYearDto),
//     AbrdnActiveSriCautious(StrategyMonthYearDto),
//     AbrdnActiveSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnActiveSriModerate(StrategyMonthYearDto),
//     AbrdnActiveSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnActiveSriAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeCautious(StrategyMonthYearDto),
//     AbrdnSippPrimeCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeSriCautious(StrategyMonthYearDto),
//     AbrdnSippPrimeSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeSriModerate(StrategyMonthYearDto),
//     AbrdnSippPrimeSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippPrimeSriAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveCautious(StrategyMonthYearDto),
//     AbrdnSippActiveCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippActiveModerate(StrategyMonthYearDto),
//     AbrdnSippActiveModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveSriCautious(StrategyMonthYearDto),
//     AbrdnSippActiveSriCautiousToModerate(StrategyMonthYearDto),
//     AbrdnSippActiveSriModerate(StrategyMonthYearDto),
//     AbrdnSippActiveSriModerateToAdventurous(StrategyMonthYearDto),
//     AbrdnSippActiveSriAdventurous(StrategyMonthYearDto),
// }



