use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashSet,
  fmt::{self, Display, Formatter}
};
use urlencoding::encode;

/// Predefined set of allowed color values in hex format.
///
/// These are the only colors supported by the Wallhaven API.
/// Colors must be provided in hex format (with or without the # prefix).
pub const COLORS: &[&str] = &[
  "#660000", "#990000", "#cc0000", "#cc3333", "#ea4c88", "#993399", "#663399",
  "#333399", "#0066cc", "#0099cc", "#66cccc", "#77cc33", "#669900", "#336600",
  "#666600", "#999900", "#cccc33", "#ffff00", "#ffcc33", "#ff9900", "#ff6600",
  "#cc6633", "#996633", "#663300", "#000000", "#999999", "#cccccc", "#ffffff",
  "#424153"
];

// -- Enums for Type-Safe Search Parameters --

/// URL struct for handling Wallhaven endpoints
///
/// Represents a Wallhaven URL with its interface type and full address.
/// Used to manage different endpoint URLs (web interface vs API) in a type-safe
/// way.
///
/// # Fields
///
/// * `interface` - The interface type (Web or API)
/// * `address` - The complete URL string
///
/// # Examples
///
/// ```rust
/// // Create a default web interface URL
/// let url = Url::default();
/// assert_eq!(url.address, "https://wallhaven.cc/search?");
///
/// // Create an API endpoint URL
/// let url = Url::new(Interface::Api);
/// assert_eq!(url.address, "https://wallhaven.cc/api/v1/search?");
/// ```

#[derive(Debug, Clone)]
pub struct Url {
  pub interface: Interface,
  pub address: String
}

impl Default for Url {
  fn default() -> Self {
    Self::new(Interface::default())
  }
}

impl Url {
  /// Creates a new URL instance with the specified interface type
  ///
  /// # Arguments
  ///
  /// * `interface` - The interface type to use (Web or API)
  ///
  /// # Returns
  ///
  /// A new Url instance with the corresponding address
  pub fn new(interface: Interface) -> Self {
    let address = interface.to_string();
    Self { interface, address }
  }

  /// Updates the URL with a new interface type
  ///
  /// # Arguments
  ///
  /// * `interface` - The new interface type to use
  ///
  /// # Returns
  ///
  /// Self with updated interface and address
  pub fn with_interface(mut self, interface: Interface) -> Self {
    self.interface = interface;
    self.address = interface.to_string();
    self
  }

  pub fn with_address(mut self, address: String) -> Self {
    self.address = address;
    self
  }
}

/// URL type for Wallhaven requests.
///
/// Determines whether to generate URLs for the web interface or API endpoints.
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum Interface {
  /// Web interface URL (https://wallhaven.cc/search?)
  #[default]
  Web,
  /// API endpoint URL (https://wallhaven.cc/api/v1/search?)
  Api
}

impl Display for Interface {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Interface::Web => "https://wallhaven.cc/search?",
        Interface::Api => "https://wallhaven.cc/api/v1/search?"
      }
    )
  }
}
