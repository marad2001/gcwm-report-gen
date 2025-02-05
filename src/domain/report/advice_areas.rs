use serde::{Deserialize, Serialize};

use crate::{domain::constrained_types::{constrained_string_1000::ConstrainedString1000, constrained_string_20::ConstrainedString20}, driving::data_transfer_object::report_type_data_transfer_object::advice_areas::{AdviceAreaDto, AdviceAreasDto}};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdviceAreas(Vec<AdviceArea>);

impl TryFrom<AdviceAreasDto> for AdviceAreas {
    type Error = String;

    fn try_from(unvalidated_advice_areas: AdviceAreasDto) -> Result<Self, Self::Error> {
        let mut validated_advice_areas = Vec::new();
        for unvalidated_advice_area in unvalidated_advice_areas.value() {
            validated_advice_areas.push(
                AdviceArea::try_from(unvalidated_advice_area)?
            )
        }
        Ok(AdviceAreas(validated_advice_areas))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AdviceArea {
    Iht(IhtAdvice),
    Will(WillAdvice),
    EmergencyFund(EmergencyFundAdvice),
    Poa(PoaAdvice),
    Other(OtherAdvice)
}

impl TryFrom<&AdviceAreaDto> for AdviceArea {
    type Error = String;

    fn try_from(unvalidated_advice_area: &AdviceAreaDto) -> Result<Self, Self::Error> {
        match unvalidated_advice_area {
            AdviceAreaDto::EmergencyFund(unvalidated_emergency_fund_advice) => {
                Ok(Self::EmergencyFund(
                    EmergencyFundAdvice { 
                        advice: ConstrainedString1000::try_from(unvalidated_emergency_fund_advice.advice.as_str())? 
                    }
                ))
            }
            AdviceAreaDto::Iht(unvalidated_iht_advice) => {
                Ok(Self::Iht(
                    IhtAdvice { 
                        advice: ConstrainedString1000::try_from(unvalidated_iht_advice.advice.as_str())? 
                    }
                ))
            }
            AdviceAreaDto::Will(unvalidated_will_advice) => {
                Ok(Self::Will(
                    WillAdvice { 
                        advice: ConstrainedString1000::try_from(unvalidated_will_advice.advice.as_str())? 
                    }
                ))
            }
            AdviceAreaDto::Poa(unvalidated_poa_advice) => {
                Ok(Self::Poa(
                    PoaAdvice { 
                        advice: ConstrainedString1000::try_from(unvalidated_poa_advice.advice.as_str())? 
                    }
                ))
            }
            AdviceAreaDto::Other(unvalidated_other_advice) => {
                Ok(Self::Other(
                    OtherAdvice { 
                        advice_description: ConstrainedString20::try_from(unvalidated_other_advice.advice.as_str())?,
                        advice: ConstrainedString1000::try_from(unvalidated_other_advice.advice.as_str())? 
                    }
                ))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IhtAdvice {
    advice: ConstrainedString1000
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WillAdvice {
    advice: ConstrainedString1000
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyFundAdvice {
    advice: ConstrainedString1000
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoaAdvice {
    advice: ConstrainedString1000
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherAdvice {
    advice_description: ConstrainedString20,
    advice: ConstrainedString1000
}