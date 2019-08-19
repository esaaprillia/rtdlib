
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a local file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // localFile
  /// Local path to the locally available file part; may be empty.
  path: Option<String>,
  /// True, if it is possible to try to download or generate the file.
  can_be_downloaded: Option<bool>,
  /// True, if the file can be deleted.
  can_be_deleted: Option<bool>,
  /// True, if the file is currently being downloaded (or a local copy is being generated by some other means).
  is_downloading_active: Option<bool>,
  /// True, if the local copy is fully available.
  is_downloading_completed: Option<bool>,
  /// Download will be started from this offset. downloaded_prefix_size is calculated from this offset.
  download_offset: Option<i32>,
  /// If is_downloading_completed is false, then only some prefix of the file starting from download_offset is ready to be read. downloaded_prefix_size is the size of that prefix.
  downloaded_prefix_size: Option<i32>,
  /// Total downloaded file bytes. Should be used only for calculating download progress. The actual file size may be bigger, and some parts of it may contain garbage.
  downloaded_size: Option<i32>,
  
}



impl Object for LocalFile {}
impl RObject for LocalFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "localFile" }
  fn td_type(&self) -> RTDType { RTDType::LocalFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LocalFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "localFile".to_string(),
      path: None,
      can_be_downloaded: None,
      can_be_deleted: None,
      is_downloading_active: None,
      is_downloading_completed: None,
      download_offset: None,
      downloaded_prefix_size: None,
      downloaded_size: None,
      
    }
  }
  
  pub fn path(&self) -> Option<String> { self.path.clone() }
  #[doc(hidden)] pub fn _set_path(&mut self, path: String) -> &mut Self { self.path = Some(path); self }
  
  pub fn can_be_downloaded(&self) -> Option<bool> { self.can_be_downloaded.clone() }
  #[doc(hidden)] pub fn _set_can_be_downloaded(&mut self, can_be_downloaded: bool) -> &mut Self { self.can_be_downloaded = Some(can_be_downloaded); self }
  
  pub fn can_be_deleted(&self) -> Option<bool> { self.can_be_deleted.clone() }
  #[doc(hidden)] pub fn _set_can_be_deleted(&mut self, can_be_deleted: bool) -> &mut Self { self.can_be_deleted = Some(can_be_deleted); self }
  
  pub fn is_downloading_active(&self) -> Option<bool> { self.is_downloading_active.clone() }
  #[doc(hidden)] pub fn _set_is_downloading_active(&mut self, is_downloading_active: bool) -> &mut Self { self.is_downloading_active = Some(is_downloading_active); self }
  
  pub fn is_downloading_completed(&self) -> Option<bool> { self.is_downloading_completed.clone() }
  #[doc(hidden)] pub fn _set_is_downloading_completed(&mut self, is_downloading_completed: bool) -> &mut Self { self.is_downloading_completed = Some(is_downloading_completed); self }
  
  pub fn download_offset(&self) -> Option<i32> { self.download_offset.clone() }
  #[doc(hidden)] pub fn _set_download_offset(&mut self, download_offset: i32) -> &mut Self { self.download_offset = Some(download_offset); self }
  
  pub fn downloaded_prefix_size(&self) -> Option<i32> { self.downloaded_prefix_size.clone() }
  #[doc(hidden)] pub fn _set_downloaded_prefix_size(&mut self, downloaded_prefix_size: i32) -> &mut Self { self.downloaded_prefix_size = Some(downloaded_prefix_size); self }
  
  pub fn downloaded_size(&self) -> Option<i32> { self.downloaded_size.clone() }
  #[doc(hidden)] pub fn _set_downloaded_size(&mut self, downloaded_size: i32) -> &mut Self { self.downloaded_size = Some(downloaded_size); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



