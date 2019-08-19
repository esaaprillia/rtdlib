
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat contains pornographic messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReportReasonPornography {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatReportReasonPornography
  
}



impl Object for ChatReportReasonPornography {}
impl RObject for ChatReportReasonPornography {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportReasonPornography" }
  fn td_type(&self) -> RTDType { RTDType::ChatReportReasonPornography }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatReportReason for ChatReportReasonPornography {}


impl ChatReportReasonPornography {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatReportReasonPornography".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



