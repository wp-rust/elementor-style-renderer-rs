use crate::transformer::{Transformer, str_field};
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;
use std::collections::HashMap;

/// Trims the span string value.
pub struct SpanTransformer;

impl Transformer for SpanTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        match value.as_str() {
            Some(s) => {
                let t = s.trim();
                if t.is_empty() { Resolved::None } else { Resolved::Single(t.to_string()) }
            }
            None => Resolved::None,
        }
    }
}

/// Ensures font-family is always quoted.
pub struct FontFamilyTransformer;

impl Transformer for FontFamilyTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let s = match value.as_str() {
            Some(s) => s.trim(),
            None => return Resolved::None,
        };
        if s.is_empty() {
            return Resolved::None;
        }
        let is_quoted = (s.starts_with('"') && s.ends_with('"'))
            || (s.starts_with('\'') && s.ends_with('\''));
        if is_quoted {
            Resolved::Single(s.to_string())
        } else {
            Resolved::Single(format!("\"{}\"", s))
        }
    }
}

/// `{x, y}` → `"x y"` (for `background-position` / `object-position`).
pub struct PositionTransformer;

impl Transformer for PositionTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let x = str_field(value, "x").unwrap_or("0px");
        let y = str_field(value, "y").unwrap_or("0px");
        Resolved::Single(format!("{} {}", x, y))
    }
}

/// `{x, y}` → `"x y"` (for `perspective-origin`).
pub struct PerspectiveOriginTransformer;

impl Transformer for PerspectiveOriginTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let x = str_field(value, "x").unwrap_or("0px");
        let y = str_field(value, "y").unwrap_or("0px");
        Resolved::Single(format!("{} {}", x, y))
    }
}

/// `{color, offset}` → `"#fff 25%"` (gradient color stop).
pub struct ColorStopTransformer;

impl Transformer for ColorStopTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let color = match str_field(value, "color") {
            Some(c) => c,
            None => return Resolved::None,
        };
        let offset = value.get("offset").and_then(|v| v.as_f64()).unwrap_or(0.0);
        Resolved::Single(format!("{} {}%", color, offset))
    }
}

/// Stroke → emits three CSS properties via Multi.
pub struct StrokeTransformer;

impl Transformer for StrokeTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let width = str_field(value, "width").unwrap_or("").to_string();
        let color = str_field(value, "color").unwrap_or("").to_string();

        if width.is_empty() && color.is_empty() {
            return Resolved::None;
        }

        let mut map = HashMap::new();
        map.insert("-webkit-text-stroke".to_string(), Some(format!("{} {}", width, color)));
        map.insert("stroke".to_string(), Some(color.clone()));
        map.insert("stroke-width".to_string(), Some(width.clone()));

        Resolved::Multi(
            map.into_iter()
                .filter_map(|(k, v)| v.map(|val| (k, val)))
                .collect(),
        )
    }
}

const ALLOWED_TRANSITION_PROPERTIES: &[&str] = &["all"];

/// Array of `{selection: {value: "all"}, size: "0.3s"}` objects → `"all 0.3s, ..."`.
pub struct TransitionTransformer;

impl Transformer for TransitionTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let arr = match value.as_array() {
            Some(a) => a,
            None => return Resolved::Single(String::new()),
        };

        let parts: Vec<String> = arr
            .iter()
            .filter_map(|t| transition_to_str(t))
            .collect();

        if parts.is_empty() {
            Resolved::Single(String::new())
        } else {
            Resolved::Single(parts.join(", "))
        }
    }
}

fn transition_to_str(t: &Value) -> Option<String> {
    let selection = t.get("selection")?;
    let size = str_field(t, "size").filter(|s| !s.is_empty())?;
    let property = str_field(selection, "value").filter(|s| !s.is_empty())?;

    if !ALLOWED_TRANSITION_PROPERTIES.contains(&property) {
        return None;
    }

    Some(format!("{} {}", property, size).trim().to_string())
}
