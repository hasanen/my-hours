use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;

/// Helper macro for creating typed strings
///
/// The macro takes one argumenta:
///
///  1. Name of the `String` type to be made
///
/// And provides a `String` wrapped inside `Struct` type.
///
#[macro_export]
macro_rules! strict_string {
    ( $name:ident) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new<S: Into<String>>(input: S) -> Self {
                Self(input.into())
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl From<String> for $name {
          fn from(val: String) -> Self {
              Self(val)
          }
        }

        impl fmt::Display for $name {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              write!(f, "{}", self.0)
          }
        }
      
    };
}

strict_string!(Fullname);
strict_string!(WorkspaceName);
strict_string!(Email);
strict_string!(ApiKey);
strict_string!(Description);
strict_string!(ClientName);
strict_string!(ProjectName);
strict_string!(ProjectHash);
strict_string!(FilePath);