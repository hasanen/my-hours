use serde::{Deserialize, Serialize};
use std::hash::Hash;
use core::hash::Hasher;
use std::cmp::Ordering;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fullname(pub String);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceName(pub String);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Email(pub String);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiKey(pub String);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description(pub String);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientName(pub String);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectName(pub String);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectHash(pub String);

impl ApiKey {
  pub fn as_str(&self) -> &str {
      &self.0
  }
}
impl WorkspaceName {
  pub fn as_str(&self) -> &str {
      &self.0
  }
}
impl Fullname {
  pub fn as_str(&self) -> &str {
      &self.0
  }
}
impl Email {
  pub fn as_str(&self) -> &str {
      &self.0
  }
}
impl ProjectName {
  pub fn as_str(&self) -> &str {
      &self.0
  }
}
impl Into<ApiKey> for String {
  fn into(self) -> ApiKey {
      ApiKey(self)
  }
}

impl Eq for Description {}
impl Eq for ProjectName {}
impl Eq for ProjectHash {}
impl Eq for ClientName {}

impl PartialEq for Description {
  fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
  }
}

impl PartialEq for ProjectName {
  fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
  }
}
impl PartialEq for ProjectHash {
  fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
  }
}
impl PartialEq for ClientName {
  fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
  }
}

impl Hash for Description {
  fn hash<H: Hasher>(&self, state: &mut H) {
      self.0.hash(state);
  }
}
impl Hash for ProjectName {
  fn hash<H: Hasher>(&self, state: &mut H) {
      self.0.hash(state);
  }
}
impl Hash for ProjectHash {
  fn hash<H: Hasher>(&self, state: &mut H) {
      self.0.hash(state);
  }
}
impl Hash for ClientName {
  fn hash<H: Hasher>(&self, state: &mut H) {
      self.0.hash(state);
  }
}
impl PartialOrd for ProjectName {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      self.0.partial_cmp(&other.0)
  }
}

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ClientName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}