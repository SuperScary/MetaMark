//! Metadata parsing for MetaMark documents.
//!
//! This module handles parsing of document metadata from YAML or TOML frontmatter
//! sections. It supports automatic format detection and conversion of metadata
//! values into a common representation.

use crate::ast::{Metadata, MetaValue};
use crate::error::{MetaMarkError, MetaMarkResult};
use serde_yaml;
use std::collections::HashMap;
use toml;

/// Parses document metadata from a string in either YAML or TOML format.
///
/// This function attempts to parse the input as YAML first, falling back to TOML
/// if YAML parsing fails. This allows for flexible metadata format support while
/// maintaining compatibility with both common formats.
///
/// # Arguments
///
/// * `content` - The string containing metadata in either YAML or TOML format
///
/// # Returns
///
/// * `Ok(Metadata)` - Successfully parsed metadata
/// * `Err(MetaMarkError)` - If parsing fails in both formats
///
/// # Example
///
/// ```
/// use metamark_core::metadata::parse_metadata;
///
/// let yaml = r#"
/// title: My Document
/// author: John Doe
/// tags:
///   - rust
///   - documentation
/// "#;
///
/// let metadata = parse_metadata(yaml).unwrap();
/// ```
pub fn parse_metadata(content: &str) -> MetaMarkResult<Metadata> {
    // Try YAML first
    match serde_yaml::from_str::<HashMap<String, serde_yaml::Value>>(content) {
        Ok(yaml_data) => Ok(Metadata {
            data: convert_yaml_to_meta_values(yaml_data),
        }),
        Err(_) => {
            // Try TOML as fallback
            match toml::from_str::<HashMap<String, toml::Value>>(content) {
                Ok(toml_data) => Ok(Metadata {
                    data: convert_toml_to_meta_values(toml_data),
                }),
                Err(e) => Err(MetaMarkError::MetadataError(format!(
                    "Failed to parse metadata as YAML or TOML: {}",
                    e
                ))),
            }
        }
    }
}

/// Converts YAML values into MetaMark's internal metadata value representation.
///
/// # Arguments
///
/// * `yaml` - HashMap of YAML values to convert
///
/// # Returns
///
/// HashMap of converted MetaValue instances
fn convert_yaml_to_meta_values(
    yaml: HashMap<String, serde_yaml::Value>,
) -> HashMap<String, MetaValue> {
    yaml.into_iter()
        .map(|(k, v)| (k, convert_yaml_value(v)))
        .collect()
}

/// Converts a single YAML value into a MetaValue.
///
/// # Arguments
///
/// * `value` - YAML value to convert
///
/// # Returns
///
/// Equivalent MetaValue instance
fn convert_yaml_value(value: serde_yaml::Value) -> MetaValue {
    match value {
        serde_yaml::Value::String(s) => MetaValue::String(s),
        serde_yaml::Value::Number(n) => MetaValue::Number(n.as_f64().unwrap_or(0.0)),
        serde_yaml::Value::Bool(b) => MetaValue::Boolean(b),
        serde_yaml::Value::Sequence(seq) => {
            MetaValue::Array(seq.into_iter().map(convert_yaml_value).collect())
        }
        serde_yaml::Value::Mapping(map) => MetaValue::Object(
            map.into_iter()
                .map(|(k, v)| {
                    (
                        k.as_str().unwrap_or_default().to_string(),
                        convert_yaml_value(v),
                    )
                })
                .collect(),
        ),
        _ => MetaValue::String("".to_string()),
    }
}

/// Converts TOML values into MetaMark's internal metadata value representation.
///
/// # Arguments
///
/// * `toml` - HashMap of TOML values to convert
///
/// # Returns
///
/// HashMap of converted MetaValue instances
fn convert_toml_to_meta_values(toml: HashMap<String, toml::Value>) -> HashMap<String, MetaValue> {
    toml.into_iter()
        .map(|(k, v)| (k, convert_toml_value(v)))
        .collect()
}

/// Converts a single TOML value into a MetaValue.
///
/// # Arguments
///
/// * `value` - TOML value to convert
///
/// # Returns
///
/// Equivalent MetaValue instance
fn convert_toml_value(value: toml::Value) -> MetaValue {
    match value {
        toml::Value::String(s) => MetaValue::String(s),
        toml::Value::Integer(i) => MetaValue::Number(i as f64),
        toml::Value::Float(f) => MetaValue::Number(f),
        toml::Value::Boolean(b) => MetaValue::Boolean(b),
        toml::Value::Array(arr) => {
            MetaValue::Array(arr.into_iter().map(convert_toml_value).collect())
        }
        toml::Value::Table(table) => MetaValue::Object(
            table
                .into_iter()
                .map(|(k, v)| (k, convert_toml_value(v)))
                .collect(),
        ),
        toml::Value::Datetime(dt) => MetaValue::String(dt.to_string()),
    }
} 