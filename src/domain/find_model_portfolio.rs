use crate::{driven::repository::{FindModelPortfolio, InvestmentPortfoliosRepository, RepoSelectError, Repository}, driving::data_transfer_object::report_type_data_transfer_object::{investment_holdings::{InvestmentStrategyProductTypeDto, InvestmentStrategyProviderDto, InvestmentStrategyServicePropositionDto, MonthYearDto}, risk_assessment_dto::RiskProfileDto}};

use super::report::investment_holdings::InvestmentPortfolio;

#[derive(Debug)]
pub enum FindOneError {
    Unknown(String),
    NotFound,
}

pub async fn find_one_model_portfolio<'a, R: InvestmentPortfoliosRepository<InvestmentPortfolio>>(
    repo: &R,
    find_model_portfolio: FindModelPortfolio
) -> Result<InvestmentPortfolio, FindOneError> {

    repo.find_one_model_portfolio(find_model_portfolio).await
        .map_err(|e| return match e {
            RepoSelectError::Unknown(e) => FindOneError::Unknown(format!("Unknown error: {}", e)),
            RepoSelectError::NotFound => FindOneError::NotFound
        })

}