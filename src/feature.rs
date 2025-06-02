//! This module provides types and utilities for working with features in the `mvt-reader` crate.
//!
//! A feature represents a geographic entity with geometry and associated properties. Features are typically found within layers of a vector tile.
//!
//! # Types
//!
//! The `feature` module defines the following types:
//!
//! - `Feature`: Represents a feature with geometry, an optional id and optional properties.

use std::collections::HashMap;

use geo_types::Geometry;

/// An enumeration representing the value of a property associated with a feature.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
  String(String),
  Float(f32),
  Double(f64),
  Int(i64),
  UInt(u64),
  SInt(i64),
  Bool(bool),
  Null,
}

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for Value {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Value::String(v) => serializer.serialize_str(v),
      Value::Float(v) => serializer.serialize_f32(*v),
      Value::Double(v) => serializer.serialize_f64(*v),
      Value::Int(v) => serializer.serialize_i64(*v),
      Value::SInt(v) => serializer.serialize_i64(*v),
      Value::UInt(v) => serializer.serialize_u64(*v),
      Value::Bool(v) => serializer.serialize_bool(*v),
      Value::Null => serializer.serialize_none(),
    }
  }
}

/// A structure representing a feature in a vector tile.
#[derive(Debug, Clone)]
pub struct Feature {
  /// The geometry of the feature.
  pub geometry: Geometry<f32>,

  /// Optional identifier for the feature.
  pub id: Option<u64>,

  /// Optional properties associated with the feature.
  pub properties: Option<HashMap<String, Value>>,
}

impl Feature {
  /// Retrieves the geometry of the feature.
  ///
  /// # Returns
  ///
  /// A reference to the geometry of the feature.
  ///
  /// # Examples
  ///
  /// ```
  /// use mvt_reader::feature::Feature;
  /// use geo_types::{Geometry, Point};
  ///
  /// let feature = Feature {
  ///   geometry: Geometry::Point(Point::new(0.0, 0.0)),
  ///   id: None,
  ///   properties: None,
  /// };
  ///
  /// let geometry = feature.get_geometry();
  /// println!("{:?}", geometry);
  /// ```
  pub fn get_geometry(&self) -> &Geometry<f32> {
    &self.geometry
  }
}
