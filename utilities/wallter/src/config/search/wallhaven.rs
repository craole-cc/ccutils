use crate::api::wallhaven::{Order, Sorting, ToplistRange};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Wallhaven-specific search parameters for the configuration.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Params {
  /// Default search query. Example: "nature", "id:123"
  pub query: Option<String>,

  /// Categories (General, Anime, People).
  pub categories: Option<(bool, bool, bool)>,

  /// Purity (SFW, Sketchy, NSFW).
  pub purity: Option<(bool, bool, bool)>,

  /// Default sorting method.
  pub sorting: Option<Sorting>,

  /// Default sorting order.
  pub order: Option<Order>,

  /// Time range for toplist sorting.
  pub top_range: Option<ToplistRange>,

  /// Minimum resolution. Example: "1920x1080".
  pub atleast: Option<String>,

  /// List of exact resolutions. Example: "1920x1080,2560x1440".
  pub resolutions: Option<String>,

  /// List of aspect ratios. Example: "16x9".
  pub ratios: Option<String>,

  /// Search by color hex code. Example: "663399".
  pub colors: Option<String>
}

impl Display for Params {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    const PAD: usize = 22;
    const TAB: usize = 6;

    printf!(
      f,
      "Default Query",
      self.query.as_deref().unwrap_or("[None]"),
      PAD,
      TAB
    )?;

    if let Some(cats) = self.categories {
      let cat_str = format!(
        "G:{} A:{} P:{}",
        if cats.0 { "✓" } else { "✗" },
        if cats.1 { "✓" } else { "✗" },
        if cats.2 { "✓" } else { "✗" }
      );
      printf!(f, "Categories", cat_str, PAD, TAB)?;
    }

    if let Some(purs) = self.purity {
      let pur_str = format!(
        "SFW:{} Sketchy:{} NSFW:{}",
        if purs.0 { "✓" } else { "✗" },
        if purs.1 { "✓" } else { "✗" },
        if purs.2 { "✓" } else { "✗" }
      );
      printf!(f, "Purity", pur_str, PAD, TAB)?;
    }

    if let Some(sorting) = self.sorting {
      printf!(f, "Sorting", format!("{sorting:?}"), PAD, TAB)?;
    }

    if let Some(order) = self.order {
      printf!(f, "Order", format!("{order:?}"), PAD, TAB)?;
    }

    if let Some(range) = self.top_range {
      printf!(f, "Top Range", format!("{range:?}"), PAD, TAB)?;
    }

    if let Some(res) = &self.atleast {
      printf!(f, "Min Resolution", res, PAD, TAB)?;
    }

    if let Some(res) = &self.resolutions {
      printf!(f, "Exact Resolutions", res, PAD, TAB)?;
    }

    if let Some(ratio) = &self.ratios {
      printf!(f, "Aspect Ratios", ratio, PAD, TAB)?;
    }

    if let Some(color) = &self.colors {
      printf!(f, "Color", color, PAD, TAB)?;
    }

    Ok(())
  }
}
