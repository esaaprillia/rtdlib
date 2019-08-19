
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the draft message in a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatDraftMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setChatDraftMessage
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New draft message; may be null.
  draft_message: Option<DraftMessage>,
  
}



impl Object for SetChatDraftMessage {}
impl RObject for SetChatDraftMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatDraftMessage" }
  fn td_type(&self) -> RTDType { RTDType::SetChatDraftMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetChatDraftMessage {}


impl SetChatDraftMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setChatDraftMessage".to_string(),
      chat_id: None,
      draft_message: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn draft_message(&self) -> Option<DraftMessage> { self.draft_message.clone() }
  #[doc(hidden)] pub fn _set_draft_message(&mut self, draft_message: DraftMessage) -> &mut Self { self.draft_message = Some(draft_message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



