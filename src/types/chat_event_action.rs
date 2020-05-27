
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a chat event
pub trait TDChatEventAction: Debug + RObject {}

/// Represents a chat event
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatEventAction {
  #[doc(hidden)] _Default(()),
  /// The chat description was changed
  ChatEventDescriptionChanged(ChatEventDescriptionChanged),
  /// The anyone_can_invite setting of a supergroup chat was toggled
  ChatEventInvitesToggled(ChatEventInvitesToggled),
  /// The is_all_history_available setting of a supergroup was toggled
  ChatEventIsAllHistoryAvailableToggled(ChatEventIsAllHistoryAvailableToggled),
  /// A new chat member was invited
  ChatEventMemberInvited(ChatEventMemberInvited),
  /// A new member joined the chat
  ChatEventMemberJoined(ChatEventMemberJoined),
  /// A member left the chat
  ChatEventMemberLeft(ChatEventMemberLeft),
  /// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
  ChatEventMemberPromoted(ChatEventMemberPromoted),
  /// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
  ChatEventMemberRestricted(ChatEventMemberRestricted),
  /// A message was deleted
  ChatEventMessageDeleted(ChatEventMessageDeleted),
  /// A message was edited
  ChatEventMessageEdited(ChatEventMessageEdited),
  /// A message was pinned
  ChatEventMessagePinned(ChatEventMessagePinned),
  /// A message was unpinned
  ChatEventMessageUnpinned(ChatEventMessageUnpinned),
  /// The chat photo was changed
  ChatEventPhotoChanged(ChatEventPhotoChanged),
  /// The sign_messages setting of a channel was toggled
  ChatEventSignMessagesToggled(ChatEventSignMessagesToggled),
  /// The supergroup sticker set was changed
  ChatEventStickerSetChanged(ChatEventStickerSetChanged),
  /// The chat title was changed
  ChatEventTitleChanged(ChatEventTitleChanged),
  /// The chat username was changed
  ChatEventUsernameChanged(ChatEventUsernameChanged),

}

impl Default for ChatEventAction {
  fn default() -> Self { ChatEventAction::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatEventAction {
  fn deserialize<D>(deserializer: D) -> Result<ChatEventAction, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatEventAction,
      (chatEventDescriptionChanged, ChatEventDescriptionChanged);
      (chatEventInvitesToggled, ChatEventInvitesToggled);
      (chatEventIsAllHistoryAvailableToggled, ChatEventIsAllHistoryAvailableToggled);
      (chatEventMemberInvited, ChatEventMemberInvited);
      (chatEventMemberJoined, ChatEventMemberJoined);
      (chatEventMemberLeft, ChatEventMemberLeft);
      (chatEventMemberPromoted, ChatEventMemberPromoted);
      (chatEventMemberRestricted, ChatEventMemberRestricted);
      (chatEventMessageDeleted, ChatEventMessageDeleted);
      (chatEventMessageEdited, ChatEventMessageEdited);
      (chatEventMessagePinned, ChatEventMessagePinned);
      (chatEventMessageUnpinned, ChatEventMessageUnpinned);
      (chatEventPhotoChanged, ChatEventPhotoChanged);
      (chatEventSignMessagesToggled, ChatEventSignMessagesToggled);
      (chatEventStickerSetChanged, ChatEventStickerSetChanged);
      (chatEventTitleChanged, ChatEventTitleChanged);
      (chatEventUsernameChanged, ChatEventUsernameChanged);

    )(deserializer)
  }
}

impl RObject for ChatEventAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatEventAction::ChatEventDescriptionChanged(t) => t.td_name(),
      ChatEventAction::ChatEventInvitesToggled(t) => t.td_name(),
      ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) => t.td_name(),
      ChatEventAction::ChatEventMemberInvited(t) => t.td_name(),
      ChatEventAction::ChatEventMemberJoined(t) => t.td_name(),
      ChatEventAction::ChatEventMemberLeft(t) => t.td_name(),
      ChatEventAction::ChatEventMemberPromoted(t) => t.td_name(),
      ChatEventAction::ChatEventMemberRestricted(t) => t.td_name(),
      ChatEventAction::ChatEventMessageDeleted(t) => t.td_name(),
      ChatEventAction::ChatEventMessageEdited(t) => t.td_name(),
      ChatEventAction::ChatEventMessagePinned(t) => t.td_name(),
      ChatEventAction::ChatEventMessageUnpinned(t) => t.td_name(),
      ChatEventAction::ChatEventPhotoChanged(t) => t.td_name(),
      ChatEventAction::ChatEventSignMessagesToggled(t) => t.td_name(),
      ChatEventAction::ChatEventStickerSetChanged(t) => t.td_name(),
      ChatEventAction::ChatEventTitleChanged(t) => t.td_name(),
      ChatEventAction::ChatEventUsernameChanged(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatEventAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatEventAction::_Default(_) = self { true } else { false } }

  pub fn is_chat_event_description_changed(&self) -> bool { if let ChatEventAction::ChatEventDescriptionChanged(_) = self { true } else { false } }
  pub fn is_chat_event_invites_toggled(&self) -> bool { if let ChatEventAction::ChatEventInvitesToggled(_) = self { true } else { false } }
  pub fn is_chat_event_is_all_history_available_toggled(&self) -> bool { if let ChatEventAction::ChatEventIsAllHistoryAvailableToggled(_) = self { true } else { false } }
  pub fn is_chat_event_member_invited(&self) -> bool { if let ChatEventAction::ChatEventMemberInvited(_) = self { true } else { false } }
  pub fn is_chat_event_member_joined(&self) -> bool { if let ChatEventAction::ChatEventMemberJoined(_) = self { true } else { false } }
  pub fn is_chat_event_member_left(&self) -> bool { if let ChatEventAction::ChatEventMemberLeft(_) = self { true } else { false } }
  pub fn is_chat_event_member_promoted(&self) -> bool { if let ChatEventAction::ChatEventMemberPromoted(_) = self { true } else { false } }
  pub fn is_chat_event_member_restricted(&self) -> bool { if let ChatEventAction::ChatEventMemberRestricted(_) = self { true } else { false } }
  pub fn is_chat_event_message_deleted(&self) -> bool { if let ChatEventAction::ChatEventMessageDeleted(_) = self { true } else { false } }
  pub fn is_chat_event_message_edited(&self) -> bool { if let ChatEventAction::ChatEventMessageEdited(_) = self { true } else { false } }
  pub fn is_chat_event_message_pinned(&self) -> bool { if let ChatEventAction::ChatEventMessagePinned(_) = self { true } else { false } }
  pub fn is_chat_event_message_unpinned(&self) -> bool { if let ChatEventAction::ChatEventMessageUnpinned(_) = self { true } else { false } }
  pub fn is_chat_event_photo_changed(&self) -> bool { if let ChatEventAction::ChatEventPhotoChanged(_) = self { true } else { false } }
  pub fn is_chat_event_sign_messages_toggled(&self) -> bool { if let ChatEventAction::ChatEventSignMessagesToggled(_) = self { true } else { false } }
  pub fn is_chat_event_sticker_set_changed(&self) -> bool { if let ChatEventAction::ChatEventStickerSetChanged(_) = self { true } else { false } }
  pub fn is_chat_event_title_changed(&self) -> bool { if let ChatEventAction::ChatEventTitleChanged(_) = self { true } else { false } }
  pub fn is_chat_event_username_changed(&self) -> bool { if let ChatEventAction::ChatEventUsernameChanged(_) = self { true } else { false } }

  pub fn on_chat_event_description_changed<F: FnOnce(&ChatEventDescriptionChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventDescriptionChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_invites_toggled<F: FnOnce(&ChatEventInvitesToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventInvitesToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_is_all_history_available_toggled<F: FnOnce(&ChatEventIsAllHistoryAvailableToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_invited<F: FnOnce(&ChatEventMemberInvited)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberInvited(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_joined<F: FnOnce(&ChatEventMemberJoined)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberJoined(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_left<F: FnOnce(&ChatEventMemberLeft)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberLeft(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_promoted<F: FnOnce(&ChatEventMemberPromoted)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberPromoted(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_restricted<F: FnOnce(&ChatEventMemberRestricted)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberRestricted(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_deleted<F: FnOnce(&ChatEventMessageDeleted)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessageDeleted(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_edited<F: FnOnce(&ChatEventMessageEdited)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessageEdited(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_pinned<F: FnOnce(&ChatEventMessagePinned)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessagePinned(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_unpinned<F: FnOnce(&ChatEventMessageUnpinned)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessageUnpinned(t) = self { fnc(t) }; self }
  pub fn on_chat_event_photo_changed<F: FnOnce(&ChatEventPhotoChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventPhotoChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_sign_messages_toggled<F: FnOnce(&ChatEventSignMessagesToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventSignMessagesToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_sticker_set_changed<F: FnOnce(&ChatEventStickerSetChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventStickerSetChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_title_changed<F: FnOnce(&ChatEventTitleChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventTitleChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_username_changed<F: FnOnce(&ChatEventUsernameChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventUsernameChanged(t) = self { fnc(t) }; self }

  pub fn as_chat_event_description_changed(&self) -> Option<&ChatEventDescriptionChanged> { if let ChatEventAction::ChatEventDescriptionChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_invites_toggled(&self) -> Option<&ChatEventInvitesToggled> { if let ChatEventAction::ChatEventInvitesToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_is_all_history_available_toggled(&self) -> Option<&ChatEventIsAllHistoryAvailableToggled> { if let ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_invited(&self) -> Option<&ChatEventMemberInvited> { if let ChatEventAction::ChatEventMemberInvited(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_joined(&self) -> Option<&ChatEventMemberJoined> { if let ChatEventAction::ChatEventMemberJoined(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_left(&self) -> Option<&ChatEventMemberLeft> { if let ChatEventAction::ChatEventMemberLeft(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_promoted(&self) -> Option<&ChatEventMemberPromoted> { if let ChatEventAction::ChatEventMemberPromoted(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_restricted(&self) -> Option<&ChatEventMemberRestricted> { if let ChatEventAction::ChatEventMemberRestricted(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_deleted(&self) -> Option<&ChatEventMessageDeleted> { if let ChatEventAction::ChatEventMessageDeleted(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_edited(&self) -> Option<&ChatEventMessageEdited> { if let ChatEventAction::ChatEventMessageEdited(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_pinned(&self) -> Option<&ChatEventMessagePinned> { if let ChatEventAction::ChatEventMessagePinned(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_unpinned(&self) -> Option<&ChatEventMessageUnpinned> { if let ChatEventAction::ChatEventMessageUnpinned(t) = self { return Some(t) } None }
  pub fn as_chat_event_photo_changed(&self) -> Option<&ChatEventPhotoChanged> { if let ChatEventAction::ChatEventPhotoChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_sign_messages_toggled(&self) -> Option<&ChatEventSignMessagesToggled> { if let ChatEventAction::ChatEventSignMessagesToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_sticker_set_changed(&self) -> Option<&ChatEventStickerSetChanged> { if let ChatEventAction::ChatEventStickerSetChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_title_changed(&self) -> Option<&ChatEventTitleChanged> { if let ChatEventAction::ChatEventTitleChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_username_changed(&self) -> Option<&ChatEventUsernameChanged> { if let ChatEventAction::ChatEventUsernameChanged(t) = self { return Some(t) } None }



  pub fn chat_event_description_changed<T: AsRef<ChatEventDescriptionChanged>>(t: T) -> Self { ChatEventAction::ChatEventDescriptionChanged(t.as_ref().clone()) }

  pub fn chat_event_invites_toggled<T: AsRef<ChatEventInvitesToggled>>(t: T) -> Self { ChatEventAction::ChatEventInvitesToggled(t.as_ref().clone()) }

  pub fn chat_event_is_all_history_available_toggled<T: AsRef<ChatEventIsAllHistoryAvailableToggled>>(t: T) -> Self { ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t.as_ref().clone()) }

  pub fn chat_event_member_invited<T: AsRef<ChatEventMemberInvited>>(t: T) -> Self { ChatEventAction::ChatEventMemberInvited(t.as_ref().clone()) }

  pub fn chat_event_member_joined<T: AsRef<ChatEventMemberJoined>>(t: T) -> Self { ChatEventAction::ChatEventMemberJoined(t.as_ref().clone()) }

  pub fn chat_event_member_left<T: AsRef<ChatEventMemberLeft>>(t: T) -> Self { ChatEventAction::ChatEventMemberLeft(t.as_ref().clone()) }

  pub fn chat_event_member_promoted<T: AsRef<ChatEventMemberPromoted>>(t: T) -> Self { ChatEventAction::ChatEventMemberPromoted(t.as_ref().clone()) }

  pub fn chat_event_member_restricted<T: AsRef<ChatEventMemberRestricted>>(t: T) -> Self { ChatEventAction::ChatEventMemberRestricted(t.as_ref().clone()) }

  pub fn chat_event_message_deleted<T: AsRef<ChatEventMessageDeleted>>(t: T) -> Self { ChatEventAction::ChatEventMessageDeleted(t.as_ref().clone()) }

  pub fn chat_event_message_edited<T: AsRef<ChatEventMessageEdited>>(t: T) -> Self { ChatEventAction::ChatEventMessageEdited(t.as_ref().clone()) }

  pub fn chat_event_message_pinned<T: AsRef<ChatEventMessagePinned>>(t: T) -> Self { ChatEventAction::ChatEventMessagePinned(t.as_ref().clone()) }

  pub fn chat_event_message_unpinned<T: AsRef<ChatEventMessageUnpinned>>(t: T) -> Self { ChatEventAction::ChatEventMessageUnpinned(t.as_ref().clone()) }

  pub fn chat_event_photo_changed<T: AsRef<ChatEventPhotoChanged>>(t: T) -> Self { ChatEventAction::ChatEventPhotoChanged(t.as_ref().clone()) }

  pub fn chat_event_sign_messages_toggled<T: AsRef<ChatEventSignMessagesToggled>>(t: T) -> Self { ChatEventAction::ChatEventSignMessagesToggled(t.as_ref().clone()) }

  pub fn chat_event_sticker_set_changed<T: AsRef<ChatEventStickerSetChanged>>(t: T) -> Self { ChatEventAction::ChatEventStickerSetChanged(t.as_ref().clone()) }

  pub fn chat_event_title_changed<T: AsRef<ChatEventTitleChanged>>(t: T) -> Self { ChatEventAction::ChatEventTitleChanged(t.as_ref().clone()) }

  pub fn chat_event_username_changed<T: AsRef<ChatEventUsernameChanged>>(t: T) -> Self { ChatEventAction::ChatEventUsernameChanged(t.as_ref().clone()) }

}

impl AsRef<ChatEventAction> for ChatEventAction {
  fn as_ref(&self) -> &ChatEventAction { self }
}







/// The chat description was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventDescriptionChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Previous chat description
  old_description: String,
  /// New chat description
  new_description: String,
  
}

impl RObject for ChatEventDescriptionChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventDescriptionChanged" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventDescriptionChanged {}



impl ChatEventDescriptionChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventDescriptionChangedBuilder {
    let mut inner = ChatEventDescriptionChanged::default();
    inner.td_name = "chatEventDescriptionChanged".to_string();
    RTDChatEventDescriptionChangedBuilder { inner }
  }

  pub fn old_description(&self) -> &String { &self.old_description }

  pub fn new_description(&self) -> &String { &self.new_description }

}

#[doc(hidden)]
pub struct RTDChatEventDescriptionChangedBuilder {
  inner: ChatEventDescriptionChanged
}

impl RTDChatEventDescriptionChangedBuilder {
  pub fn build(&self) -> ChatEventDescriptionChanged { self.inner.clone() }

   
  pub fn old_description<T: AsRef<str>>(&mut self, old_description: T) -> &mut Self {
    self.inner.old_description = old_description.as_ref().to_string();
    self
  }

   
  pub fn new_description<T: AsRef<str>>(&mut self, new_description: T) -> &mut Self {
    self.inner.new_description = new_description.as_ref().to_string();
    self
  }

}

impl AsRef<ChatEventDescriptionChanged> for ChatEventDescriptionChanged {
  fn as_ref(&self) -> &ChatEventDescriptionChanged { self }
}

impl AsRef<ChatEventDescriptionChanged> for RTDChatEventDescriptionChangedBuilder {
  fn as_ref(&self) -> &ChatEventDescriptionChanged { &self.inner }
}







/// The anyone_can_invite setting of a supergroup chat was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventInvitesToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// New value of anyone_can_invite
  anyone_can_invite: bool,
  
}

impl RObject for ChatEventInvitesToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventInvitesToggled" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventInvitesToggled {}



impl ChatEventInvitesToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventInvitesToggledBuilder {
    let mut inner = ChatEventInvitesToggled::default();
    inner.td_name = "chatEventInvitesToggled".to_string();
    RTDChatEventInvitesToggledBuilder { inner }
  }

  pub fn anyone_can_invite(&self) -> bool { self.anyone_can_invite }

}

#[doc(hidden)]
pub struct RTDChatEventInvitesToggledBuilder {
  inner: ChatEventInvitesToggled
}

impl RTDChatEventInvitesToggledBuilder {
  pub fn build(&self) -> ChatEventInvitesToggled { self.inner.clone() }

   
  pub fn anyone_can_invite(&mut self, anyone_can_invite: bool) -> &mut Self {
    self.inner.anyone_can_invite = anyone_can_invite;
    self
  }

}

impl AsRef<ChatEventInvitesToggled> for ChatEventInvitesToggled {
  fn as_ref(&self) -> &ChatEventInvitesToggled { self }
}

impl AsRef<ChatEventInvitesToggled> for RTDChatEventInvitesToggledBuilder {
  fn as_ref(&self) -> &ChatEventInvitesToggled { &self.inner }
}







/// The is_all_history_available setting of a supergroup was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventIsAllHistoryAvailableToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// New value of is_all_history_available
  is_all_history_available: bool,
  
}

impl RObject for ChatEventIsAllHistoryAvailableToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventIsAllHistoryAvailableToggled" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventIsAllHistoryAvailableToggled {}



impl ChatEventIsAllHistoryAvailableToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventIsAllHistoryAvailableToggledBuilder {
    let mut inner = ChatEventIsAllHistoryAvailableToggled::default();
    inner.td_name = "chatEventIsAllHistoryAvailableToggled".to_string();
    RTDChatEventIsAllHistoryAvailableToggledBuilder { inner }
  }

  pub fn is_all_history_available(&self) -> bool { self.is_all_history_available }

}

#[doc(hidden)]
pub struct RTDChatEventIsAllHistoryAvailableToggledBuilder {
  inner: ChatEventIsAllHistoryAvailableToggled
}

impl RTDChatEventIsAllHistoryAvailableToggledBuilder {
  pub fn build(&self) -> ChatEventIsAllHistoryAvailableToggled { self.inner.clone() }

   
  pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
    self.inner.is_all_history_available = is_all_history_available;
    self
  }

}

impl AsRef<ChatEventIsAllHistoryAvailableToggled> for ChatEventIsAllHistoryAvailableToggled {
  fn as_ref(&self) -> &ChatEventIsAllHistoryAvailableToggled { self }
}

impl AsRef<ChatEventIsAllHistoryAvailableToggled> for RTDChatEventIsAllHistoryAvailableToggledBuilder {
  fn as_ref(&self) -> &ChatEventIsAllHistoryAvailableToggled { &self.inner }
}







/// A new chat member was invited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberInvited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// New member user identifier
  user_id: i64,
  /// New member status
  status: ChatMemberStatus,
  
}

impl RObject for ChatEventMemberInvited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberInvited" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberInvited {}



impl ChatEventMemberInvited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberInvitedBuilder {
    let mut inner = ChatEventMemberInvited::default();
    inner.td_name = "chatEventMemberInvited".to_string();
    RTDChatEventMemberInvitedBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn status(&self) -> &ChatMemberStatus { &self.status }

}

#[doc(hidden)]
pub struct RTDChatEventMemberInvitedBuilder {
  inner: ChatEventMemberInvited
}

impl RTDChatEventMemberInvitedBuilder {
  pub fn build(&self) -> ChatEventMemberInvited { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMemberInvited> for ChatEventMemberInvited {
  fn as_ref(&self) -> &ChatEventMemberInvited { self }
}

impl AsRef<ChatEventMemberInvited> for RTDChatEventMemberInvitedBuilder {
  fn as_ref(&self) -> &ChatEventMemberInvited { &self.inner }
}







/// A new member joined the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberJoined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatEventMemberJoined {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberJoined" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberJoined {}



impl ChatEventMemberJoined {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberJoinedBuilder {
    let mut inner = ChatEventMemberJoined::default();
    inner.td_name = "chatEventMemberJoined".to_string();
    RTDChatEventMemberJoinedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatEventMemberJoinedBuilder {
  inner: ChatEventMemberJoined
}

impl RTDChatEventMemberJoinedBuilder {
  pub fn build(&self) -> ChatEventMemberJoined { self.inner.clone() }

}

impl AsRef<ChatEventMemberJoined> for ChatEventMemberJoined {
  fn as_ref(&self) -> &ChatEventMemberJoined { self }
}

impl AsRef<ChatEventMemberJoined> for RTDChatEventMemberJoinedBuilder {
  fn as_ref(&self) -> &ChatEventMemberJoined { &self.inner }
}







/// A member left the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatEventMemberLeft {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberLeft" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberLeft {}



impl ChatEventMemberLeft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberLeftBuilder {
    let mut inner = ChatEventMemberLeft::default();
    inner.td_name = "chatEventMemberLeft".to_string();
    RTDChatEventMemberLeftBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatEventMemberLeftBuilder {
  inner: ChatEventMemberLeft
}

impl RTDChatEventMemberLeftBuilder {
  pub fn build(&self) -> ChatEventMemberLeft { self.inner.clone() }

}

impl AsRef<ChatEventMemberLeft> for ChatEventMemberLeft {
  fn as_ref(&self) -> &ChatEventMemberLeft { self }
}

impl AsRef<ChatEventMemberLeft> for RTDChatEventMemberLeftBuilder {
  fn as_ref(&self) -> &ChatEventMemberLeft { &self.inner }
}







/// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberPromoted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Chat member user identifier
  user_id: i64,
  /// Previous status of the chat member
  old_status: ChatMemberStatus,
  /// New status of the chat member
  new_status: ChatMemberStatus,
  
}

impl RObject for ChatEventMemberPromoted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberPromoted" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberPromoted {}



impl ChatEventMemberPromoted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberPromotedBuilder {
    let mut inner = ChatEventMemberPromoted::default();
    inner.td_name = "chatEventMemberPromoted".to_string();
    RTDChatEventMemberPromotedBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn old_status(&self) -> &ChatMemberStatus { &self.old_status }

  pub fn new_status(&self) -> &ChatMemberStatus { &self.new_status }

}

#[doc(hidden)]
pub struct RTDChatEventMemberPromotedBuilder {
  inner: ChatEventMemberPromoted
}

impl RTDChatEventMemberPromotedBuilder {
  pub fn build(&self) -> ChatEventMemberPromoted { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn old_status<T: AsRef<ChatMemberStatus>>(&mut self, old_status: T) -> &mut Self {
    self.inner.old_status = old_status.as_ref().clone();
    self
  }

   
  pub fn new_status<T: AsRef<ChatMemberStatus>>(&mut self, new_status: T) -> &mut Self {
    self.inner.new_status = new_status.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMemberPromoted> for ChatEventMemberPromoted {
  fn as_ref(&self) -> &ChatEventMemberPromoted { self }
}

impl AsRef<ChatEventMemberPromoted> for RTDChatEventMemberPromotedBuilder {
  fn as_ref(&self) -> &ChatEventMemberPromoted { &self.inner }
}







/// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Chat member user identifier
  user_id: i64,
  /// Previous status of the chat member
  old_status: ChatMemberStatus,
  /// New status of the chat member
  new_status: ChatMemberStatus,
  
}

impl RObject for ChatEventMemberRestricted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberRestricted" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberRestricted {}



impl ChatEventMemberRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberRestrictedBuilder {
    let mut inner = ChatEventMemberRestricted::default();
    inner.td_name = "chatEventMemberRestricted".to_string();
    RTDChatEventMemberRestrictedBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn old_status(&self) -> &ChatMemberStatus { &self.old_status }

  pub fn new_status(&self) -> &ChatMemberStatus { &self.new_status }

}

#[doc(hidden)]
pub struct RTDChatEventMemberRestrictedBuilder {
  inner: ChatEventMemberRestricted
}

impl RTDChatEventMemberRestrictedBuilder {
  pub fn build(&self) -> ChatEventMemberRestricted { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn old_status<T: AsRef<ChatMemberStatus>>(&mut self, old_status: T) -> &mut Self {
    self.inner.old_status = old_status.as_ref().clone();
    self
  }

   
  pub fn new_status<T: AsRef<ChatMemberStatus>>(&mut self, new_status: T) -> &mut Self {
    self.inner.new_status = new_status.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMemberRestricted> for ChatEventMemberRestricted {
  fn as_ref(&self) -> &ChatEventMemberRestricted { self }
}

impl AsRef<ChatEventMemberRestricted> for RTDChatEventMemberRestrictedBuilder {
  fn as_ref(&self) -> &ChatEventMemberRestricted { &self.inner }
}







/// A message was deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Deleted message
  message: Message,
  
}

impl RObject for ChatEventMessageDeleted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageDeleted" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessageDeleted {}



impl ChatEventMessageDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessageDeletedBuilder {
    let mut inner = ChatEventMessageDeleted::default();
    inner.td_name = "chatEventMessageDeleted".to_string();
    RTDChatEventMessageDeletedBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDChatEventMessageDeletedBuilder {
  inner: ChatEventMessageDeleted
}

impl RTDChatEventMessageDeletedBuilder {
  pub fn build(&self) -> ChatEventMessageDeleted { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMessageDeleted> for ChatEventMessageDeleted {
  fn as_ref(&self) -> &ChatEventMessageDeleted { self }
}

impl AsRef<ChatEventMessageDeleted> for RTDChatEventMessageDeletedBuilder {
  fn as_ref(&self) -> &ChatEventMessageDeleted { &self.inner }
}







/// A message was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageEdited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The original message before the edit
  old_message: Message,
  /// The message after it was edited
  new_message: Message,
  
}

impl RObject for ChatEventMessageEdited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageEdited" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessageEdited {}



impl ChatEventMessageEdited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessageEditedBuilder {
    let mut inner = ChatEventMessageEdited::default();
    inner.td_name = "chatEventMessageEdited".to_string();
    RTDChatEventMessageEditedBuilder { inner }
  }

  pub fn old_message(&self) -> &Message { &self.old_message }

  pub fn new_message(&self) -> &Message { &self.new_message }

}

#[doc(hidden)]
pub struct RTDChatEventMessageEditedBuilder {
  inner: ChatEventMessageEdited
}

impl RTDChatEventMessageEditedBuilder {
  pub fn build(&self) -> ChatEventMessageEdited { self.inner.clone() }

   
  pub fn old_message<T: AsRef<Message>>(&mut self, old_message: T) -> &mut Self {
    self.inner.old_message = old_message.as_ref().clone();
    self
  }

   
  pub fn new_message<T: AsRef<Message>>(&mut self, new_message: T) -> &mut Self {
    self.inner.new_message = new_message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMessageEdited> for ChatEventMessageEdited {
  fn as_ref(&self) -> &ChatEventMessageEdited { self }
}

impl AsRef<ChatEventMessageEdited> for RTDChatEventMessageEditedBuilder {
  fn as_ref(&self) -> &ChatEventMessageEdited { &self.inner }
}







/// A message was pinned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessagePinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Pinned message
  message: Message,
  
}

impl RObject for ChatEventMessagePinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessagePinned" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessagePinned {}



impl ChatEventMessagePinned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessagePinnedBuilder {
    let mut inner = ChatEventMessagePinned::default();
    inner.td_name = "chatEventMessagePinned".to_string();
    RTDChatEventMessagePinnedBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDChatEventMessagePinnedBuilder {
  inner: ChatEventMessagePinned
}

impl RTDChatEventMessagePinnedBuilder {
  pub fn build(&self) -> ChatEventMessagePinned { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMessagePinned> for ChatEventMessagePinned {
  fn as_ref(&self) -> &ChatEventMessagePinned { self }
}

impl AsRef<ChatEventMessagePinned> for RTDChatEventMessagePinnedBuilder {
  fn as_ref(&self) -> &ChatEventMessagePinned { &self.inner }
}







/// A message was unpinned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageUnpinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatEventMessageUnpinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageUnpinned" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessageUnpinned {}



impl ChatEventMessageUnpinned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessageUnpinnedBuilder {
    let mut inner = ChatEventMessageUnpinned::default();
    inner.td_name = "chatEventMessageUnpinned".to_string();
    RTDChatEventMessageUnpinnedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatEventMessageUnpinnedBuilder {
  inner: ChatEventMessageUnpinned
}

impl RTDChatEventMessageUnpinnedBuilder {
  pub fn build(&self) -> ChatEventMessageUnpinned { self.inner.clone() }

}

impl AsRef<ChatEventMessageUnpinned> for ChatEventMessageUnpinned {
  fn as_ref(&self) -> &ChatEventMessageUnpinned { self }
}

impl AsRef<ChatEventMessageUnpinned> for RTDChatEventMessageUnpinnedBuilder {
  fn as_ref(&self) -> &ChatEventMessageUnpinned { &self.inner }
}







/// The chat photo was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventPhotoChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Previous chat photo value; may be null
  old_photo: Option<ChatPhoto>,
  /// New chat photo value; may be null
  new_photo: Option<ChatPhoto>,
  
}

impl RObject for ChatEventPhotoChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventPhotoChanged" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventPhotoChanged {}



impl ChatEventPhotoChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventPhotoChangedBuilder {
    let mut inner = ChatEventPhotoChanged::default();
    inner.td_name = "chatEventPhotoChanged".to_string();
    RTDChatEventPhotoChangedBuilder { inner }
  }

  pub fn old_photo(&self) -> &Option<ChatPhoto> { &self.old_photo }

  pub fn new_photo(&self) -> &Option<ChatPhoto> { &self.new_photo }

}

#[doc(hidden)]
pub struct RTDChatEventPhotoChangedBuilder {
  inner: ChatEventPhotoChanged
}

impl RTDChatEventPhotoChangedBuilder {
  pub fn build(&self) -> ChatEventPhotoChanged { self.inner.clone() }

   
  pub fn old_photo<T: AsRef<ChatPhoto>>(&mut self, old_photo: T) -> &mut Self {
    self.inner.old_photo = Some(old_photo.as_ref().clone());
    self
  }

   
  pub fn new_photo<T: AsRef<ChatPhoto>>(&mut self, new_photo: T) -> &mut Self {
    self.inner.new_photo = Some(new_photo.as_ref().clone());
    self
  }

}

impl AsRef<ChatEventPhotoChanged> for ChatEventPhotoChanged {
  fn as_ref(&self) -> &ChatEventPhotoChanged { self }
}

impl AsRef<ChatEventPhotoChanged> for RTDChatEventPhotoChangedBuilder {
  fn as_ref(&self) -> &ChatEventPhotoChanged { &self.inner }
}







/// The sign_messages setting of a channel was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventSignMessagesToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// New value of sign_messages
  sign_messages: bool,
  
}

impl RObject for ChatEventSignMessagesToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventSignMessagesToggled" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventSignMessagesToggled {}



impl ChatEventSignMessagesToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventSignMessagesToggledBuilder {
    let mut inner = ChatEventSignMessagesToggled::default();
    inner.td_name = "chatEventSignMessagesToggled".to_string();
    RTDChatEventSignMessagesToggledBuilder { inner }
  }

  pub fn sign_messages(&self) -> bool { self.sign_messages }

}

#[doc(hidden)]
pub struct RTDChatEventSignMessagesToggledBuilder {
  inner: ChatEventSignMessagesToggled
}

impl RTDChatEventSignMessagesToggledBuilder {
  pub fn build(&self) -> ChatEventSignMessagesToggled { self.inner.clone() }

   
  pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
    self.inner.sign_messages = sign_messages;
    self
  }

}

impl AsRef<ChatEventSignMessagesToggled> for ChatEventSignMessagesToggled {
  fn as_ref(&self) -> &ChatEventSignMessagesToggled { self }
}

impl AsRef<ChatEventSignMessagesToggled> for RTDChatEventSignMessagesToggledBuilder {
  fn as_ref(&self) -> &ChatEventSignMessagesToggled { &self.inner }
}







/// The supergroup sticker set was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventStickerSetChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Previous identifier of the chat sticker set; 0 if none
  old_sticker_set_id: isize,
  /// New identifier of the chat sticker set; 0 if none
  new_sticker_set_id: isize,
  
}

impl RObject for ChatEventStickerSetChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventStickerSetChanged" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventStickerSetChanged {}



impl ChatEventStickerSetChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventStickerSetChangedBuilder {
    let mut inner = ChatEventStickerSetChanged::default();
    inner.td_name = "chatEventStickerSetChanged".to_string();
    RTDChatEventStickerSetChangedBuilder { inner }
  }

  pub fn old_sticker_set_id(&self) -> isize { self.old_sticker_set_id }

  pub fn new_sticker_set_id(&self) -> isize { self.new_sticker_set_id }

}

#[doc(hidden)]
pub struct RTDChatEventStickerSetChangedBuilder {
  inner: ChatEventStickerSetChanged
}

impl RTDChatEventStickerSetChangedBuilder {
  pub fn build(&self) -> ChatEventStickerSetChanged { self.inner.clone() }

   
  pub fn old_sticker_set_id(&mut self, old_sticker_set_id: isize) -> &mut Self {
    self.inner.old_sticker_set_id = old_sticker_set_id;
    self
  }

   
  pub fn new_sticker_set_id(&mut self, new_sticker_set_id: isize) -> &mut Self {
    self.inner.new_sticker_set_id = new_sticker_set_id;
    self
  }

}

impl AsRef<ChatEventStickerSetChanged> for ChatEventStickerSetChanged {
  fn as_ref(&self) -> &ChatEventStickerSetChanged { self }
}

impl AsRef<ChatEventStickerSetChanged> for RTDChatEventStickerSetChangedBuilder {
  fn as_ref(&self) -> &ChatEventStickerSetChanged { &self.inner }
}







/// The chat title was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventTitleChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Previous chat title
  old_title: String,
  /// New chat title
  new_title: String,
  
}

impl RObject for ChatEventTitleChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventTitleChanged" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventTitleChanged {}



impl ChatEventTitleChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventTitleChangedBuilder {
    let mut inner = ChatEventTitleChanged::default();
    inner.td_name = "chatEventTitleChanged".to_string();
    RTDChatEventTitleChangedBuilder { inner }
  }

  pub fn old_title(&self) -> &String { &self.old_title }

  pub fn new_title(&self) -> &String { &self.new_title }

}

#[doc(hidden)]
pub struct RTDChatEventTitleChangedBuilder {
  inner: ChatEventTitleChanged
}

impl RTDChatEventTitleChangedBuilder {
  pub fn build(&self) -> ChatEventTitleChanged { self.inner.clone() }

   
  pub fn old_title<T: AsRef<str>>(&mut self, old_title: T) -> &mut Self {
    self.inner.old_title = old_title.as_ref().to_string();
    self
  }

   
  pub fn new_title<T: AsRef<str>>(&mut self, new_title: T) -> &mut Self {
    self.inner.new_title = new_title.as_ref().to_string();
    self
  }

}

impl AsRef<ChatEventTitleChanged> for ChatEventTitleChanged {
  fn as_ref(&self) -> &ChatEventTitleChanged { self }
}

impl AsRef<ChatEventTitleChanged> for RTDChatEventTitleChangedBuilder {
  fn as_ref(&self) -> &ChatEventTitleChanged { &self.inner }
}







/// The chat username was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventUsernameChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Previous chat username
  old_username: String,
  /// New chat username
  new_username: String,
  
}

impl RObject for ChatEventUsernameChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventUsernameChanged" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventUsernameChanged {}



impl ChatEventUsernameChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventUsernameChangedBuilder {
    let mut inner = ChatEventUsernameChanged::default();
    inner.td_name = "chatEventUsernameChanged".to_string();
    RTDChatEventUsernameChangedBuilder { inner }
  }

  pub fn old_username(&self) -> &String { &self.old_username }

  pub fn new_username(&self) -> &String { &self.new_username }

}

#[doc(hidden)]
pub struct RTDChatEventUsernameChangedBuilder {
  inner: ChatEventUsernameChanged
}

impl RTDChatEventUsernameChangedBuilder {
  pub fn build(&self) -> ChatEventUsernameChanged { self.inner.clone() }

   
  pub fn old_username<T: AsRef<str>>(&mut self, old_username: T) -> &mut Self {
    self.inner.old_username = old_username.as_ref().to_string();
    self
  }

   
  pub fn new_username<T: AsRef<str>>(&mut self, new_username: T) -> &mut Self {
    self.inner.new_username = new_username.as_ref().to_string();
    self
  }

}

impl AsRef<ChatEventUsernameChanged> for ChatEventUsernameChanged {
  fn as_ref(&self) -> &ChatEventUsernameChanged { self }
}

impl AsRef<ChatEventUsernameChanged> for RTDChatEventUsernameChangedBuilder {
  fn as_ref(&self) -> &ChatEventUsernameChanged { &self.inner }
}



