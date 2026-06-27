use crate::transformer::{Transformer, f64_field, str_field};
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;

/// `{"size": N, "unit": "px"}` → `"16px"` / `"auto"` / raw custom string.
pub struct SizeTransformer;

impl Transformer for SizeTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let unit = str_field(value, "unit").unwrap_or("px");

        if unit == "custom" {
            return match str_field(value, "size") {
                Some(s) => Resolved::Single(s.to_string()),
                None => Resolved::None,
            };
        }

        if unit == "auto" {
            return Resolved::Single("auto".to_string());
        }

        match f64_field(value, "size") {
            Some(n) => {
                let s = if n.fract() == 0.0 {
                    format!("{}{}", n as i64, unit)
                } else {
                    format!("{}{}", n, unit)
                };
                Resolved::Single(s)
            }
            None => Resolved::None,
        }
    }
}

/// Grid track variant: `fr` units use `repeat(N, 1fr)`.
pub struct GridTrackSizeTransformer;

impl Transformer for GridTrackSizeTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let unit = str_field(value, "unit").unwrap_or("px");

        if unit == "custom" {
            return match str_field(value, "size") {
                Some(s) => Resolved::Single(s.to_string()),
                None => Resolved::None,
            };
        }

        if unit == "fr" {
            let count = f64_field(value, "size").unwrap_or(1.0) as i64;
            if count < 1 {
                return Resolved::None;
            }
            return Resolved::Single(format!("repeat({}, 1fr)", count));
        }

        // delegate to normal size logic
        SizeTransformer.transform(value, _ctx)
    }
}
