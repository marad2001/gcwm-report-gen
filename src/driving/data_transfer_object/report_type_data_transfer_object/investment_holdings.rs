use serde::{Deserialize, Serialize};

use super::risk_assessment_dto::RiskProfileDto;


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum InvestmentStrategyDto {
    PastInvestmentStrategy(PastInvestmentStrategyDto),
    InvestableInvestmentStrategy(InvestableInvestmentStrategyDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PastInvestmentStrategyDto {
    BespokeInvestmentStrategy(InvestmentPortfolioDto),
    GCWMInvestmentStrategy(GCWMPastInvestmentStrategyDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum InvestableInvestmentStrategyDto {
    BespokeInvestmentStrategy(InvestmentPortfolioDto),
    GCWMInvestmentStrategy(GCWMPresentInvestmentStrategyDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentPortfolioDto {
    pub risk_level: RiskProfileDto,
    pub fund_holdings: Vec<FundHoldingDto>,
    pub fund_charges: Option<f32>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GCWMPastInvestmentStrategyDto {
    TransactPrimeCautious(StrategyMonthYearDto),
    TransactPrimeCautiousToModerate(StrategyMonthYearDto),
    TransactPrimeModerate(StrategyMonthYearDto),
    TransactPrimeModerateToAdventurous(StrategyMonthYearDto),
    TransactPrimeAdventurous(StrategyMonthYearDto),
    TransactPrimeSriCautious(StrategyMonthYearDto),
    TransactPrimeSriCautiousToModerate(StrategyMonthYearDto),
    TransactPrimeSriModerate(StrategyMonthYearDto),
    TransactPrimeSriModerateToAdventurous(StrategyMonthYearDto),
    TransactPrimeSriAdventurous(StrategyMonthYearDto),
    TransactActiveCautious(StrategyMonthYearDto),
    TransactActiveCautiousToModerate(StrategyMonthYearDto),
    TransactActiveModerate(StrategyMonthYearDto),
    TransactActiveModerateToAdventurous(StrategyMonthYearDto),
    TransactActiveAdventurous(StrategyMonthYearDto),
    TransactActiveSriCautious(StrategyMonthYearDto),
    TransactActiveSriCautiousToModerate(StrategyMonthYearDto),
    TransactActiveSriModerate(StrategyMonthYearDto),
    TransactActiveSriModerateToAdventurous(StrategyMonthYearDto),
    TransactActiveSriAdventurous(StrategyMonthYearDto),
    AbrdnPrimeCautious(StrategyMonthYearDto),
    AbrdnPrimeCautiousToModerate(StrategyMonthYearDto),
    AbrdnPrimeModerate(StrategyMonthYearDto),
    AbrdnPrimeModerateToAdventurous(StrategyMonthYearDto),
    AbrdnPrimeAdventurous(StrategyMonthYearDto),
    AbrdnPrimeSriCautious(StrategyMonthYearDto),
    AbrdnPrimeSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnPrimeSriModerate(StrategyMonthYearDto),
    AbrdnPrimeSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnPrimeSriAdventurous(StrategyMonthYearDto),
    AbrdnActiveCautious(StrategyMonthYearDto),
    AbrdnActiveCautiousToModerate(StrategyMonthYearDto),
    AbrdnActiveModerate(StrategyMonthYearDto),
    AbrdnActiveModerateToAdventurous(StrategyMonthYearDto),
    AbrdnActiveAdventurous(StrategyMonthYearDto),
    AbrdnActiveSriCautious(StrategyMonthYearDto),
    AbrdnActiveSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnActiveSriModerate(StrategyMonthYearDto),
    AbrdnActiveSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnActiveSriAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeCautious(StrategyMonthYearDto),
    AbrdnSippPrimeCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippPrimeModerate(StrategyMonthYearDto),
    AbrdnSippPrimeModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeSriCautious(StrategyMonthYearDto),
    AbrdnSippPrimeSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippPrimeSriModerate(StrategyMonthYearDto),
    AbrdnSippPrimeSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeSriAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveCautious(StrategyMonthYearDto),
    AbrdnSippActiveCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippActiveModerate(StrategyMonthYearDto),
    AbrdnSippActiveModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveSriCautious(StrategyMonthYearDto),
    AbrdnSippActiveSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippActiveSriModerate(StrategyMonthYearDto),
    AbrdnSippActiveSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveSriAdventurous(StrategyMonthYearDto),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GCWMPresentInvestmentStrategyDto {
    TransactPrimeCautious(StrategyMonthYearDto),
    TransactPrimeCautiousToModerate(StrategyMonthYearDto),
    TransactPrimeModerate(StrategyMonthYearDto),
    TransactPrimeModerateToAdventurous(StrategyMonthYearDto),
    TransactPrimeAdventurous(StrategyMonthYearDto),
    TransactPrimeSriCautious(StrategyMonthYearDto),
    TransactPrimeSriCautiousToModerate(StrategyMonthYearDto),
    TransactPrimeSriModerate(StrategyMonthYearDto),
    TransactPrimeSriModerateToAdventurous(StrategyMonthYearDto),
    TransactPrimeSriAdventurous(StrategyMonthYearDto),
    TransactActiveCautious(StrategyMonthYearDto),
    TransactActiveCautiousToModerate(StrategyMonthYearDto),
    TransactActiveModerate(StrategyMonthYearDto),
    TransactActiveModerateToAdventurous(StrategyMonthYearDto),
    TransactActiveAdventurous(StrategyMonthYearDto),
    TransactActiveSriCautious(StrategyMonthYearDto),
    TransactActiveSriCautiousToModerate(StrategyMonthYearDto),
    TransactActiveSriModerate(StrategyMonthYearDto),
    TransactActiveSriModerateToAdventurous(StrategyMonthYearDto),
    TransactActiveSriAdventurous(StrategyMonthYearDto),
    AbrdnPrimeCautious(StrategyMonthYearDto),
    AbrdnPrimeCautiousToModerate(StrategyMonthYearDto),
    AbrdnPrimeModerate(StrategyMonthYearDto),
    AbrdnPrimeModerateToAdventurous(StrategyMonthYearDto),
    AbrdnPrimeAdventurous(StrategyMonthYearDto),
    AbrdnPrimeSriCautious(StrategyMonthYearDto),
    AbrdnPrimeSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnPrimeSriModerate(StrategyMonthYearDto),
    AbrdnPrimeSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnPrimeSriAdventurous(StrategyMonthYearDto),
    AbrdnActiveCautious(StrategyMonthYearDto),
    AbrdnActiveCautiousToModerate(StrategyMonthYearDto),
    AbrdnActiveModerate(StrategyMonthYearDto),
    AbrdnActiveModerateToAdventurous(StrategyMonthYearDto),
    AbrdnActiveAdventurous(StrategyMonthYearDto),
    AbrdnActiveSriCautious(StrategyMonthYearDto),
    AbrdnActiveSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnActiveSriModerate(StrategyMonthYearDto),
    AbrdnActiveSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnActiveSriAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeCautious(StrategyMonthYearDto),
    AbrdnSippPrimeCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippPrimeModerate(StrategyMonthYearDto),
    AbrdnSippPrimeModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeSriCautious(StrategyMonthYearDto),
    AbrdnSippPrimeSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippPrimeSriModerate(StrategyMonthYearDto),
    AbrdnSippPrimeSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippPrimeSriAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveCautious(StrategyMonthYearDto),
    AbrdnSippActiveCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippActiveModerate(StrategyMonthYearDto),
    AbrdnSippActiveModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveSriCautious(StrategyMonthYearDto),
    AbrdnSippActiveSriCautiousToModerate(StrategyMonthYearDto),
    AbrdnSippActiveSriModerate(StrategyMonthYearDto),
    AbrdnSippActiveSriModerateToAdventurous(StrategyMonthYearDto),
    AbrdnSippActiveSriAdventurous(StrategyMonthYearDto),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum StrategyMonthYearDto {
    Aug24(InvestmentPortfolioDto)
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

