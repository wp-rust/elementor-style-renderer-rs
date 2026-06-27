use crate::transformer::{Transformer, str_field};
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;

/// Array of pre-resolved transform function strings → joined with spaces.
pub struct TransformFunctionsTransformer;

impl Transformer for TransformFunctionsTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let arr = match value.as_array() {
            Some(a) => a,
            None => return Resolved::None,
        };
        let parts: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();
        if parts.is_empty() {
            Resolved::None
        } else {
            Resolved::Single(parts.join(" "))
        }
    }
}

/// `{x, y, z}` → `translate3d(x, y, z)`.
pub struct TransformMoveTransformer;

impl Transformer for TransformMoveTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let def = "0px";
        Resolved::Single(format!(
            "translate3d({}, {}, {})",
            str_field(value, "x").unwrap_or(def),
            str_field(value, "y").unwrap_or(def),
            str_field(value, "z").unwrap_or(def),
        ))
    }
}

/// `{x, y, z}` → `rotateX(x) rotateY(y) rotateZ(z)`.
pub struct TransformRotateTransformer;

impl Transformer for TransformRotateTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let def = "0deg";
        Resolved::Single(format!(
            "rotateX({}) rotateY({}) rotateZ({})",
            str_field(value, "x").unwrap_or(def),
            str_field(value, "y").unwrap_or(def),
            str_field(value, "z").unwrap_or(def),
        ))
    }
}

/// `{x, y, z}` → `scale3d(x, y, z)`.
pub struct TransformScaleTransformer;

impl Transformer for TransformScaleTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        Resolved::Single(format!(
            "scale3d({}, {}, {})",
            value.get("x").and_then(|v| v.as_f64()).map(|n| if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{}", n) }).unwrap_or_else(|| "1".to_string()),
            value.get("y").and_then(|v| v.as_f64()).map(|n| if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{}", n) }).unwrap_or_else(|| "1".to_string()),
            value.get("z").and_then(|v| v.as_f64()).map(|n| if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{}", n) }).unwrap_or_else(|| "1".to_string()),
        ))
    }
}

/// `{x, y}` → `skew(x, y)`.
pub struct TransformSkewTransformer;

impl Transformer for TransformSkewTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let def = "0deg";
        Resolved::Single(format!(
            "skew({}, {})",
            str_field(value, "x").unwrap_or(def),
            str_field(value, "y").unwrap_or(def),
        ))
    }
}

/// `{x, y, z}` → `"x y z"`, or `None` if all values are the default (50% 50% 0px).
pub struct TransformOriginTransformer;

impl Transformer for TransformOriginTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let default_origin = "0px";
        let default_xy = "50%";

        let x = str_field(value, "x").unwrap_or(default_origin);
        let y = str_field(value, "y").unwrap_or(default_origin);
        let z = str_field(value, "z").unwrap_or(default_origin);

        if x == default_xy && y == default_xy && z == default_origin {
            return Resolved::None;
        }

        Resolved::Single(format!("{} {} {}", x, y, z))
    }
}
