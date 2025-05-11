use serde::{Deserialize, Serialize};

pub const HOST: &str = "www.hemkop.se";
pub mod product;
pub mod search;

/// Represents a Hemköp product code, e.g. "101482036_ST".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductCode(pub String);

impl std::fmt::Display for ProductCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<&str> for ProductCode {
	fn from(s: &str) -> Self {
		Self(s.to_string())
	}
}
