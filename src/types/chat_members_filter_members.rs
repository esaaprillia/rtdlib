
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all chat members, including restricted chat members. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMembersFilterMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMembersFilterMembers
  
}



impl Object for ChatMembersFilterMembers {}
impl RObject for ChatMembersFilterMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterMembers" }
  fn td_type(&self) -> RTDType { RTDType::ChatMembersFilterMembers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMembersFilter for ChatMembersFilterMembers {}


impl ChatMembersFilterMembers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMembersFilterMembers".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



