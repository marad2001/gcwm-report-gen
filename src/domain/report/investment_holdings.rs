use serde::{Deserialize, Serialize};

use crate::{
    domain::constrained_types::{
        constrained_money_amount_large::ConstrainedMoneyAmountLarge, 
        constrained_string_200::ConstrainedString200, 
        isin::ISIN, percentage::Percentage, 
        sedol::Sedol
    }, 
    driving::data_transfer_object::report_type_data_transfer_object::investment_holdings::{
        FundHoldingDto, 
        InvestableInvestmentStrategyDto, 
        InvestmentPortfolioDto, 
        InvestmentStrategyDto, 
        PastInvestmentStrategyDto, 
        StrategyMonthYearDto,
        GCWMPastInvestmentStrategyDto,
        GCWMPresentInvestmentStrategyDto
    },
};

use super::risk_assessment::RiskProfile;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum InvestmentStrategy {
    PastInvestmentStrategy(PastInvestmentStrategy),
    InvestableInvestmentStrategy(InvestableInvestmentStrategy)
}

impl Default for InvestmentStrategy {
    fn default() -> Self {
        InvestmentStrategy::InvestableInvestmentStrategy(
            InvestableInvestmentStrategy::BespokeInvestmentStrategy(
                InvestmentPortfolio {
                    risk_level: RiskProfile::Moderate,
                    fund_holdings: Vec::new(),
                    fund_charges: Percentage::default()
                }
            )
        )
    }
}

impl InvestmentStrategy {
    /// Returns the risk level from the underlying InvestmentPortfolio.
    pub fn risk_level(&self) -> &RiskProfile {
        self.investment_portfolio().risk_level()
    }

    /// Returns the fund holdings from the underlying InvestmentPortfolio.
    pub fn fund_holdings(&self) -> &Vec<FundHolding> {
        self.investment_portfolio().fund_holdings()
    }

    /// Returns the fund charges (as a Percentage) from the underlying InvestmentPortfolio.
    pub fn fund_charges(&self) -> &Percentage {
        self.investment_portfolio().fund_charges()
    }

    /// Helper method to extract the underlying InvestmentPortfolio.
    fn investment_portfolio(&self) -> &InvestmentPortfolio {
        match self {
            InvestmentStrategy::PastInvestmentStrategy(past) => match past {
                PastInvestmentStrategy::BespokeInvestmentStrategy(portfolio) => portfolio,
                PastInvestmentStrategy::GCWMInvestmentStrategy(gcwmpast) => {
                    gcwmpast.investment_portfolio()
                }
            },
            InvestmentStrategy::InvestableInvestmentStrategy(investable) => match investable {
                InvestableInvestmentStrategy::BespokeInvestmentStrategy(portfolio) => portfolio,
                InvestableInvestmentStrategy::GCWMInvestmentStrategy(gcwmpresent) => {
                    gcwmpresent.investment_portfolio()
                }
            },
        }
    }
}

impl TryFrom<InvestmentStrategyDto> for InvestmentStrategy {
    type Error = String;

    fn try_from(dto: InvestmentStrategyDto) -> Result<Self, Self::Error> {
        match dto {
            InvestmentStrategyDto::PastInvestmentStrategy(dto) => Ok(InvestmentStrategy::PastInvestmentStrategy(dto.try_into()?)),
            InvestmentStrategyDto::InvestableInvestmentStrategy(dto) => Ok(InvestmentStrategy::InvestableInvestmentStrategy(dto.try_into()?))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PastInvestmentStrategy {
    BespokeInvestmentStrategy(InvestmentPortfolio),
    GCWMInvestmentStrategy(GCWMPastInvestmentStrategy)
}

impl TryFrom<PastInvestmentStrategyDto> for PastInvestmentStrategy {
    type Error = String;

    fn try_from(dto: PastInvestmentStrategyDto) -> Result<Self, Self::Error> {
        match dto {
            PastInvestmentStrategyDto::BespokeInvestmentStrategy(dto) => Ok(PastInvestmentStrategy::BespokeInvestmentStrategy(dto.try_into()?)),
            PastInvestmentStrategyDto::GCWMInvestmentStrategy(dto) => Ok(PastInvestmentStrategy::GCWMInvestmentStrategy(dto.try_into()?))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum InvestableInvestmentStrategy {
    BespokeInvestmentStrategy(InvestmentPortfolio),
    GCWMInvestmentStrategy(GCWMPresentInvestmentStrategy)
}

impl TryFrom<InvestableInvestmentStrategyDto> for InvestableInvestmentStrategy {
    type Error = String;

    fn try_from(dto: InvestableInvestmentStrategyDto) -> Result<Self, Self::Error> {
        match dto {
            InvestableInvestmentStrategyDto::BespokeInvestmentStrategy(dto) => Ok(InvestableInvestmentStrategy::BespokeInvestmentStrategy(dto.try_into()?)),
            InvestableInvestmentStrategyDto::GCWMInvestmentStrategy(dto) => Ok(InvestableInvestmentStrategy::GCWMInvestmentStrategy(dto.try_into()?))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentPortfolio {
    risk_level: RiskProfile,
    fund_holdings: Vec<FundHolding>,
    fund_charges: Percentage
}

impl InvestmentPortfolio {
    pub fn risk_level(&self) -> &RiskProfile {
        &self.risk_level
    }
    pub fn fund_holdings(&self) -> &Vec<FundHolding> {
        &self.fund_holdings
    }
    pub fn fund_charges(&self) -> &Percentage {
        &self.fund_charges
    }
}

use std::convert::TryFrom;

impl TryFrom<InvestmentPortfolioDto> for InvestmentPortfolio {
    type Error = String;

    fn try_from(dto: InvestmentPortfolioDto) -> Result<Self, Self::Error> {
        // Convert each FundHoldingDto into FundHolding.
        let fund_holdings: Vec<FundHolding> = dto
            .fund_holdings
            .into_iter()
            .map(FundHolding::try_from)
            .collect::<Result<_, _>>()?;
        
        // Compute the weighted average fund charge.
        // We can compute the weight in one of two ways:
        // 1. If every FundHolding has a percentage_of_portfolio, use those.
        // 2. Otherwise, if every FundHolding has a value, compute weight = value / total_value.
        let computed_charge: f32 = if fund_holdings.iter().all(|fh| fh.percentage_of_portfolio.is_some()) {
            // Use the provided percentages.
            let total_percentage: f32 = fund_holdings
                .iter()
                .map(|fh| fh.percentage_of_portfolio.as_ref().unwrap().value())
                .sum();
            // Allow a small tolerance for floating‐point rounding.
            if (total_percentage - 1.0).abs() > 0.01 {
                return Err("Fund holdings percentages do not sum to 100%".to_string());
            }
            fund_holdings
                .iter()
                .map(|fh| {
                    let weight = fh.percentage_of_portfolio.as_ref().unwrap().value();
                    let charge = fh.fund_charge.value();
                    weight * charge
                })
                .sum()
        } else if fund_holdings.iter().all(|fh| fh.value.is_some()) {
            // Use the holding values to compute weights.
            let total_value: f64 = fund_holdings
                .iter()
                .map(|fh| fh.value.as_ref().unwrap().value())
                .sum();
            if total_value == 0.0 {
                return Err("Total fund holding value must be positive".to_string());
            }
            fund_holdings
                .iter()
                .map(|fh| {
                    let weight = (fh.value.as_ref().unwrap().value() / total_value) as f32;
                    let charge = fh.fund_charge.value();
                    weight * charge
                })
                .sum()
        } else {
            return Err("Fund holdings must either all have percentages or all have values".to_string());
        };

        // Convert the computed f32 into a Percentage (using its TryFrom implementation).
        let fund_charges: Percentage = computed_charge.try_into()?;

        Ok(Self {
            risk_level: dto.risk_level.try_into()?,
            fund_holdings,
            fund_charges,
        })
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GCWMPastInvestmentStrategy {
    TransactPrimeCautious(StrategyMonthYear),
    TransactPrimeCautiousToModerate(StrategyMonthYear),
    TransactPrimeModerate(StrategyMonthYear),
    TransactPrimeModerateToAdventurous(StrategyMonthYear),
    TransactPrimeAdventurous(StrategyMonthYear),
    TransactPrimeSriCautious(StrategyMonthYear),
    TransactPrimeSriCautiousToModerate(StrategyMonthYear),
    TransactPrimeSriModerate(StrategyMonthYear),
    TransactPrimeSriModerateToAdventurous(StrategyMonthYear),
    TransactPrimeSriAdventurous(StrategyMonthYear),
    TransactActiveCautious(StrategyMonthYear),
    TransactActiveCautiousToModerate(StrategyMonthYear),
    TransactActiveModerate(StrategyMonthYear),
    TransactActiveModerateToAdventurous(StrategyMonthYear),
    TransactActiveAdventurous(StrategyMonthYear),
    TransactActiveSriCautious(StrategyMonthYear),
    TransactActiveSriCautiousToModerate(StrategyMonthYear),
    TransactActiveSriModerate(StrategyMonthYear),
    TransactActiveSriModerateToAdventurous(StrategyMonthYear),
    TransactActiveSriAdventurous(StrategyMonthYear),
    AbrdnPrimeCautious(StrategyMonthYear),
    AbrdnPrimeCautiousToModerate(StrategyMonthYear),
    AbrdnPrimeModerate(StrategyMonthYear),
    AbrdnPrimeModerateToAdventurous(StrategyMonthYear),
    AbrdnPrimeAdventurous(StrategyMonthYear),
    AbrdnPrimeSriCautious(StrategyMonthYear),
    AbrdnPrimeSriCautiousToModerate(StrategyMonthYear),
    AbrdnPrimeSriModerate(StrategyMonthYear),
    AbrdnPrimeSriModerateToAdventurous(StrategyMonthYear),
    AbrdnPrimeSriAdventurous(StrategyMonthYear),
    AbrdnActiveCautious(StrategyMonthYear),
    AbrdnActiveCautiousToModerate(StrategyMonthYear),
    AbrdnActiveModerate(StrategyMonthYear),
    AbrdnActiveModerateToAdventurous(StrategyMonthYear),
    AbrdnActiveAdventurous(StrategyMonthYear),
    AbrdnActiveSriCautious(StrategyMonthYear),
    AbrdnActiveSriCautiousToModerate(StrategyMonthYear),
    AbrdnActiveSriModerate(StrategyMonthYear),
    AbrdnActiveSriModerateToAdventurous(StrategyMonthYear),
    AbrdnActiveSriAdventurous(StrategyMonthYear),
    AbrdnSippPrimeCautious(StrategyMonthYear),
    AbrdnSippPrimeCautiousToModerate(StrategyMonthYear),
    AbrdnSippPrimeModerate(StrategyMonthYear),
    AbrdnSippPrimeModerateToAdventurous(StrategyMonthYear),
    AbrdnSippPrimeAdventurous(StrategyMonthYear),
    AbrdnSippPrimeSriCautious(StrategyMonthYear),
    AbrdnSippPrimeSriCautiousToModerate(StrategyMonthYear),
    AbrdnSippPrimeSriModerate(StrategyMonthYear),
    AbrdnSippPrimeSriModerateToAdventurous(StrategyMonthYear),
    AbrdnSippPrimeSriAdventurous(StrategyMonthYear),
    AbrdnSippActiveCautious(StrategyMonthYear),
    AbrdnSippActiveCautiousToModerate(StrategyMonthYear),
    AbrdnSippActiveModerate(StrategyMonthYear),
    AbrdnSippActiveModerateToAdventurous(StrategyMonthYear),
    AbrdnSippActiveAdventurous(StrategyMonthYear),
    AbrdnSippActiveSriCautious(StrategyMonthYear),
    AbrdnSippActiveSriCautiousToModerate(StrategyMonthYear),
    AbrdnSippActiveSriModerate(StrategyMonthYear),
    AbrdnSippActiveSriModerateToAdventurous(StrategyMonthYear),
    AbrdnSippActiveSriAdventurous(StrategyMonthYear),
}

impl GCWMPastInvestmentStrategy {
    pub fn investment_portfolio(&self) -> &InvestmentPortfolio {
        match self {
            // Combine all variants in one match arm using `|` because they all contain a StrategyMonthYear.
            GCWMPastInvestmentStrategy::TransactPrimeCautious(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeAdventurous(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeSriCautious(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeSriCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeSriModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeSriModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::TransactPrimeSriAdventurous(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveCautious(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveAdventurous(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveSriCautious(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveSriCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveSriModerate(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveSriModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::TransactActiveSriAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeSriCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeSriCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeSriModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeSriModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnPrimeSriAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveSriCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveSriCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveSriModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveSriModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnActiveSriAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeSriCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeSriCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeSriModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeSriModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippPrimeSriAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveSriCautious(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveSriCautiousToModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveSriModerate(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveSriModerateToAdventurous(strategy)
            | GCWMPastInvestmentStrategy::AbrdnSippActiveSriAdventurous(strategy)
            => match strategy {
                StrategyMonthYear::Aug24(ip) => ip,
            },
        }
    }
}

impl TryFrom<GCWMPastInvestmentStrategyDto> for GCWMPastInvestmentStrategy {
    type Error = String;

    fn try_from(dto: GCWMPastInvestmentStrategyDto) -> Result<Self, Self::Error> {
        Ok(match dto {
            GCWMPastInvestmentStrategyDto::TransactPrimeCautious(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeSriCautious(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeSriCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeSriCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeSriCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeSriModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeSriModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeSriModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeSriModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactPrimeSriAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactPrimeSriAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveCautious(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveSriCautious(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveSriCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveSriCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveSriCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveSriModerate(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveSriModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveSriModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveSriModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::TransactActiveSriAdventurous(inner) =>
                GCWMPastInvestmentStrategy::TransactActiveSriAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeSriCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeSriCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeSriCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeSriCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeSriModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeSriModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeSriModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeSriModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnPrimeSriAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnPrimeSriAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveSriCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveSriCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveSriCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveSriCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveSriModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveSriModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveSriModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveSriModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnActiveSriAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnActiveSriAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeSriCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeSriCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeSriCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeSriCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeSriModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeSriModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeSriModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeSriModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippPrimeSriAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippPrimeSriAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveSriCautious(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveSriCautious(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveSriCautiousToModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveSriCautiousToModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveSriModerate(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveSriModerate(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveSriModerateToAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveSriModerateToAdventurous(inner.try_into()?),
            GCWMPastInvestmentStrategyDto::AbrdnSippActiveSriAdventurous(inner) =>
                GCWMPastInvestmentStrategy::AbrdnSippActiveSriAdventurous(inner.try_into()?),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GCWMPresentInvestmentStrategy {
    TransactPrimeCautious(StrategyMonthYear),
    TransactPrimeCautiousToModerate(StrategyMonthYear),
    TransactPrimeModerate(StrategyMonthYear),
    TransactPrimeModerateToAdventurous(StrategyMonthYear),
    TransactPrimeAdventurous(StrategyMonthYear),
    TransactPrimeSriCautious(StrategyMonthYear),
    TransactPrimeSriCautiousToModerate(StrategyMonthYear),
    TransactPrimeSriModerate(StrategyMonthYear),
    TransactPrimeSriModerateToAdventurous(StrategyMonthYear),
    TransactPrimeSriAdventurous(StrategyMonthYear),
    TransactActiveCautious(StrategyMonthYear),
    TransactActiveCautiousToModerate(StrategyMonthYear),
    TransactActiveModerate(StrategyMonthYear),
    TransactActiveModerateToAdventurous(StrategyMonthYear),
    TransactActiveAdventurous(StrategyMonthYear),
    TransactActiveSriCautious(StrategyMonthYear),
    TransactActiveSriCautiousToModerate(StrategyMonthYear),
    TransactActiveSriModerate(StrategyMonthYear),
    TransactActiveSriModerateToAdventurous(StrategyMonthYear),
    TransactActiveSriAdventurous(StrategyMonthYear),
    AbrdnPrimeCautious(StrategyMonthYear),
    AbrdnPrimeCautiousToModerate(StrategyMonthYear),
    AbrdnPrimeModerate(StrategyMonthYear),
    AbrdnPrimeModerateToAdventurous(StrategyMonthYear),
    AbrdnPrimeAdventurous(StrategyMonthYear),
    AbrdnPrimeSriCautious(StrategyMonthYear),
    AbrdnPrimeSriCautiousToModerate(StrategyMonthYear),
    AbrdnPrimeSriModerate(StrategyMonthYear),
    AbrdnPrimeSriModerateToAdventurous(StrategyMonthYear),
    AbrdnPrimeSriAdventurous(StrategyMonthYear),
    AbrdnActiveCautious(StrategyMonthYear),
    AbrdnActiveCautiousToModerate(StrategyMonthYear),
    AbrdnActiveModerate(StrategyMonthYear),
    AbrdnActiveModerateToAdventurous(StrategyMonthYear),
    AbrdnActiveAdventurous(StrategyMonthYear),
    AbrdnActiveSriCautious(StrategyMonthYear),
    AbrdnActiveSriCautiousToModerate(StrategyMonthYear),
    AbrdnActiveSriModerate(StrategyMonthYear),
    AbrdnActiveSriModerateToAdventurous(StrategyMonthYear),
    AbrdnActiveSriAdventurous(StrategyMonthYear),
    AbrdnSippPrimeCautious(StrategyMonthYear),
    AbrdnSippPrimeCautiousToModerate(StrategyMonthYear),
    AbrdnSippPrimeModerate(StrategyMonthYear),
    AbrdnSippPrimeModerateToAdventurous(StrategyMonthYear),
    AbrdnSippPrimeAdventurous(StrategyMonthYear),
    AbrdnSippPrimeSriCautious(StrategyMonthYear),
    AbrdnSippPrimeSriCautiousToModerate(StrategyMonthYear),
    AbrdnSippPrimeSriModerate(StrategyMonthYear),
    AbrdnSippPrimeSriModerateToAdventurous(StrategyMonthYear),
    AbrdnSippPrimeSriAdventurous(StrategyMonthYear),
    AbrdnSippActiveCautious(StrategyMonthYear),
    AbrdnSippActiveCautiousToModerate(StrategyMonthYear),
    AbrdnSippActiveModerate(StrategyMonthYear),
    AbrdnSippActiveModerateToAdventurous(StrategyMonthYear),
    AbrdnSippActiveAdventurous(StrategyMonthYear),
    AbrdnSippActiveSriCautious(StrategyMonthYear),
    AbrdnSippActiveSriCautiousToModerate(StrategyMonthYear),
    AbrdnSippActiveSriModerate(StrategyMonthYear),
    AbrdnSippActiveSriModerateToAdventurous(StrategyMonthYear),
    AbrdnSippActiveSriAdventurous(StrategyMonthYear),
}

impl GCWMPresentInvestmentStrategy {
    pub fn investment_portfolio(&self) -> &InvestmentPortfolio {
        match self {
            GCWMPresentInvestmentStrategy::TransactPrimeCautious(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeSriCautious(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeSriCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeSriModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeSriModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::TransactPrimeSriAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveCautious(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveSriCautious(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveSriCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveSriModerate(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveSriModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::TransactActiveSriAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeSriCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeSriCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeSriModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeSriModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnPrimeSriAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveSriCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveSriCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveSriModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveSriModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnActiveSriAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveSriCautious(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveSriCautiousToModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveSriModerate(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveSriModerateToAdventurous(strategy)
            | GCWMPresentInvestmentStrategy::AbrdnSippActiveSriAdventurous(strategy)
            => match strategy {
                StrategyMonthYear::Aug24(ip) => ip,
            },
        }
    }
}

impl TryFrom<GCWMPresentInvestmentStrategyDto> for GCWMPresentInvestmentStrategy {
    type Error = String;

    fn try_from(dto: GCWMPresentInvestmentStrategyDto) -> Result<Self, Self::Error> {
        Ok(match dto {
            GCWMPresentInvestmentStrategyDto::TransactPrimeCautious(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeSriCautious(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeSriCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeSriCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeSriCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeSriModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeSriModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeSriModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeSriModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactPrimeSriAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactPrimeSriAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveCautious(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveSriCautious(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveSriCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveSriCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveSriCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveSriModerate(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveSriModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveSriModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveSriModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::TransactActiveSriAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::TransactActiveSriAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeSriCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeSriCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeSriCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeSriCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeSriModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeSriModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeSriModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeSriModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnPrimeSriAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnPrimeSriAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveSriCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveSriCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveSriCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveSriCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveSriModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveSriModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveSriModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveSriModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnActiveSriAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnActiveSriAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeSriCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeSriCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeSriModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeSriModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippPrimeSriAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippPrimeSriAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveSriCautious(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveSriCautious(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveSriCautiousToModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveSriCautiousToModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveSriModerate(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveSriModerate(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveSriModerateToAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveSriModerateToAdventurous(inner.try_into()?),
            GCWMPresentInvestmentStrategyDto::AbrdnSippActiveSriAdventurous(inner) =>
                GCWMPresentInvestmentStrategy::AbrdnSippActiveSriAdventurous(inner.try_into()?),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum StrategyMonthYear {
    Aug24(InvestmentPortfolio)
}

impl TryFrom<StrategyMonthYearDto> for StrategyMonthYear {
    type Error = String;

    fn try_from(dto: StrategyMonthYearDto) -> Result<Self, Self::Error> {
        match dto {
            StrategyMonthYearDto::Aug24(dto) => Ok(StrategyMonthYear::Aug24(dto.try_into()?))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundHolding {
    fund_name: ConstrainedString200,
    isin: Option<ISIN>,
    sedol: Option<Sedol>,
    value: Option<ConstrainedMoneyAmountLarge>,
    percentage_of_portfolio: Option<Percentage>,
    fund_charge: Percentage
}

impl TryFrom<FundHoldingDto> for FundHolding {
    type Error = String;

    fn try_from(dto: FundHoldingDto) -> Result<Self, Self::Error> {

        if dto.value.is_none() && dto.percentage_of_portfolio.is_none() {
            return Err("A fund holding must have either a value or a percentage of the portoflio".to_string())
        }

        Ok(Self {
            fund_name: dto.fund_name.try_into()?,
            isin: if dto.isin.is_some() { Some(dto.isin.unwrap().try_into()?) } else { None },
            sedol: if dto.sedol.is_some() { Some(dto.sedol.unwrap().try_into()?) } else { None },
            value: if dto.value.is_some() { Some(dto.value.unwrap().try_into()?) } else { None },
            percentage_of_portfolio: if dto.percentage_of_portfolio.is_some() { Some(dto.percentage_of_portfolio.unwrap().try_into()?) } else { None },
            fund_charge: dto.fund_charge.try_into()?
        })
    }
}