#[cfg(test)]
mod tests;

mod from;
mod opts;
mod transform;

use crate::common::{approx::approx_def, hsl_hsv_from_str, tuple_to_string, ColorIter, Hs};
use crate::consts::RATIO_MAX;
use crate::normalize::{normalize_hue, normalize_percent, normalize_ratio};

use crate::{ColorAlpha, ColorTuple, ColorTupleA, ParseError, Rgb};

/// The HSL or HSI (hue, saturation, lightness (intensity)) color model
///
/// Ranges:
/// * h (hue): 0.0 - 360.0
/// * s (saturation): 0.0 - 100.0
/// * l (lightness): 0.0 - 100.0
/// * a (alpha): 0.0 - 1.0
#[derive(Debug, PartialEq, Clone)]
pub struct Hsl {
  h: f64,
  s: f64,
  l: f64,
  a: Option<f64>,
}

impl Hsl {
  fn _apply_tuple(&mut self, t: &ColorTuple) {
    self.h = t.0;
    self.s = t.1;
    self.l = t.2;
  }

  pub fn new(h: f64, s: f64, l: f64, a: Option<f64>) -> Hsl {
    let a = a.map(normalize_ratio).filter(|al| !approx_def(*al, RATIO_MAX));
    let np = normalize_percent;
    Hsl { h: normalize_hue(h), s: np(s), l: np(l), a }
  }

  pub fn to_css_string(&self) -> String {
    let t: ColorTupleA = self.into();
    tuple_to_string(&t, "hsl")
  }

  pub fn get_hue(&self) -> f64 {
    self.h
  }
  pub fn get_saturation(&self) -> f64 {
    self.s
  }
  pub fn get_lightness(&self) -> f64 {
    self.l
  }

  pub fn set_hue(&mut self, val: f64) {
    self.h = normalize_hue(val);
  }
  pub fn set_saturation(&mut self, val: f64) {
    self.s = normalize_percent(val);
  }
  pub fn set_lightness(&mut self, val: f64) {
    self.l = normalize_percent(val);
  }

  pub fn iter(&self) -> ColorIter {
    ColorIter::from_tuple_w_alpha(self.into(), self.a)
  }
}

//
//
//
// Default
//
impl Default for Hsl {
  fn default() -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: 0.0, a: None }
  }
}

//
//
//
// AsRef<Hsl>
//
impl AsRef<Hsl> for Hsl {
  fn as_ref(&self) -> &Hsl {
    &self
  }
}

//
//
//
// FromStr
//
impl std::str::FromStr for Hsl {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Hsl, ParseError> {
    let (tuple, alpha) = hsl_hsv_from_str(s, Hs::Hsl)?;
    let mut hsl = Hsl::from(&tuple);
    if let Some(a) = alpha {
      hsl.set_alpha(a);
    }
    Ok(hsl)
  }
}

//
//
//
// ColorAlpha
//
impl ColorAlpha for Hsl {
  fn get_alpha(&self) -> f64 {
    self.a.unwrap_or(1.0)
  }

  fn set_alpha(&mut self, val: f64) {
    self.a = Some(normalize_ratio(val));
  }

  fn opacify(&mut self, val: f64) {
    self.set_alpha(self.get_alpha() + val);
  }
}

//
//
//
// Iter
//
impl<'a> std::iter::IntoIterator for &'a Hsl {
  type Item = f64;
  type IntoIter = ColorIter;
  fn into_iter(self) -> ColorIter {
    self.iter()
  }
}

impl std::iter::IntoIterator for Hsl {
  type Item = f64;
  type IntoIter = ColorIter;
  fn into_iter(self) -> ColorIter {
    self.iter()
  }
}
