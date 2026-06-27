/// Core types shared across the style renderer.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ─── PropValue ────────────────────────────────────────────────────────────────

/// A typed Elementor PropValue: `{"$$type": "...", "value": ...}`.
/// `None` value in the map = explicit null reset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropValue {
    #[serde(rename = "$$type")]
    pub type_key: String,
    pub value: Value,
    #[serde(default)]
    pub disabled: bool,
}

/// Multi-props wrapper produced by some transformers.
/// `{"$$multi-props": true, "value": {prop: value, ...}}`
#[derive(Debug, Clone)]
pub struct MultiProps {
    pub values: HashMap<String, Option<String>>,
}

// ─── Transformer context ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct TransformerContext {
    pub key: Option<String>,
    pub disabled: bool,
}

impl TransformerContext {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: Some(key.into()),
            disabled: false,
        }
    }
}

// ─── Render output ────────────────────────────────────────────────────────────

/// The result of resolving a single prop value through the transformer chain.
#[derive(Debug, Clone)]
pub enum Resolved {
    /// A CSS value string ready to use in `property: value;`.
    Single(String),
    /// Multiple CSS properties emitted by one transformer (e.g. `stroke` → 3 props).
    Multi(HashMap<String, String>),
    /// The prop should be omitted from output.
    None,
}

// ─── Style input ─────────────────────────────────────────────────────────────

/// A style variant: one set of props + breakpoint/state metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct StyleVariant {
    pub props: HashMap<String, Value>,
    #[serde(default)]
    pub meta: VariantMeta,
    pub custom_css: Option<CustomCss>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VariantMeta {
    pub breakpoint: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CustomCss {
    pub raw: Option<String>,
}

/// A complete style definition (one element's styles).
#[derive(Debug, Clone, Deserialize)]
pub struct StyleDefinition {
    pub id: String,
    #[serde(rename = "type")]
    pub style_type: String,
    #[serde(rename = "cssName")]
    pub css_name: Option<String>,
    pub variants: Vec<StyleVariant>,
}

// ─── Breakpoint ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Breakpoint {
    pub direction: BreakpointDirection,
    pub value: u32,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
}

fn default_true() -> bool { true }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BreakpointDirection {
    Min,
    Max,
}
