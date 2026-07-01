/// `Transformer` trait — port of PHP `Transformer_Base`.
/// Each implementation converts a resolved prop `Value` into a CSS string (or multi-props map).

use crate::types::{MultiProps, Resolved, TransformerContext};
use serde_json::Value;

pub trait Transformer: Send + Sync {
    /// Transform a resolved `value` (already had object/array children resolved)
    /// into a CSS string, multi-props map, or nothing.
    fn transform(&self, value: &Value, ctx: &TransformerContext) -> Resolved;
}

/// Registry mapping `$$type` keys → transformers.
#[derive(Default)]
pub struct TransformerRegistry {
    transformers: std::collections::HashMap<String, Box<dyn Transformer>>,
    fallback: Option<Box<dyn Transformer>>,
}

impl TransformerRegistry {
    pub fn register(&mut self, key: impl Into<String>, t: impl Transformer + 'static) {
        self.transformers.insert(key.into(), Box::new(t));
    }

    pub fn register_fallback(&mut self, t: impl Transformer + 'static) {
        self.fallback = Some(Box::new(t));
    }

    pub fn get(&self, key: &str) -> Option<&dyn Transformer> {
        self.transformers
            .get(key)
            .map(|b| b.as_ref())
            .or_else(|| self.fallback.as_deref())
    }
}

/// Helper: extract `Option<String>` from a `Value` field.
pub fn str_field<'a>(obj: &'a Value, key: &str) -> Option<&'a str> {
    obj.get(key).and_then(|v| v.as_str())
}

pub fn str_field_owned(obj: &Value, key: &str) -> Option<String> {
    str_field(obj, key).map(|s| s.to_string())
}

pub fn f64_field(obj: &Value, key: &str) -> Option<f64> {
    obj.get(key).and_then(|v| v.as_f64())
}
