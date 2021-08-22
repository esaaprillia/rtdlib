
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of bot commands
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommands {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Bot's user identifier
  bot_user_id: i64,
  /// List of bot commands
  commands: Vec<BotCommand>,
  
}

impl RObject for BotCommands {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommands" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl BotCommands {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandsBuilder {
    let mut inner = BotCommands::default();
    inner.td_name = "botCommands".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandsBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn commands(&self) -> &Vec<BotCommand> { &self.commands }

}

#[doc(hidden)]
pub struct RTDBotCommandsBuilder {
  inner: BotCommands
}

impl RTDBotCommandsBuilder {
  pub fn build(&self) -> BotCommands { self.inner.clone() }

   
  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

   
  pub fn commands(&mut self, commands: Vec<BotCommand>) -> &mut Self {
    self.inner.commands = commands;
    self
  }

}

impl AsRef<BotCommands> for BotCommands {
  fn as_ref(&self) -> &BotCommands { self }
}

impl AsRef<BotCommands> for RTDBotCommandsBuilder {
  fn as_ref(&self) -> &BotCommands { &self.inner }
}



