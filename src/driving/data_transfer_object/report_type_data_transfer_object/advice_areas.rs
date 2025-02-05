use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdviceAreasDto(Vec<AdviceAreaDto>);

impl AdviceAreasDto {
    pub fn value(&self) -> &Vec<AdviceAreaDto>{
        &self.0
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum AdviceAreaDto {
    Iht(IhtAdvice),
    Will(WillAdvice),
    EmergencyFund(EmergencyFundAdvice),
    Poa(PoaAdvice),
    Other(OtherAdvice)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IhtAdvice {
    pub advice: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WillAdvice {
    pub advice: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyFundAdvice {
    pub advice: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoaAdvice {
    pub advice: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherAdvice {
    pub advice_description: String,
    pub advice: String
}