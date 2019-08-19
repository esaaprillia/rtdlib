
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the notification settings of a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setChatNotificationSettings
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New notification settings for the chat.
  notification_settings: Option<ChatNotificationSettings>,
  
}



impl Object for SetChatNotificationSettings {}
impl RObject for SetChatNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatNotificationSettings" }
  fn td_type(&self) -> RTDType { RTDType::SetChatNotificationSettings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetChatNotificationSettings {}


impl SetChatNotificationSettings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setChatNotificationSettings".to_string(),
      chat_id: None,
      notification_settings: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn notification_settings(&self) -> Option<ChatNotificationSettings> { self.notification_settings.clone() }
  #[doc(hidden)] pub fn _set_notification_settings(&mut self, notification_settings: ChatNotificationSettings) -> &mut Self { self.notification_settings = Some(notification_settings); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



