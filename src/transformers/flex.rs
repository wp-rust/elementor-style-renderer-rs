use crate::transformer::Transformer;
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;

pub struct FlexTransformer;

impl Transformer for FlexTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let grow = nonempty(value.get("flexGrow").and_then(|v| to_str(v)));
        let shrink = nonempty(value.get("flexShrink").and_then(|v| to_str(v)));
        let basis = value.get("flexBasis").and_then(|b| basis_to_str(b));
        let basis = basis.as_deref().and_then(|s| nonempty(Some(s)));

        match (grow, shrink, basis) {
            (None, None, None) => Resolved::None,
            (Some(g), Some(s), Some(b)) => Resolved::Single(format!("{} {} {}", g, s, b)),
            (Some(g), Some(s), None) => Resolved::Single(format!("{} {}", g, s)),
            (Some(g), None, Some(b)) => Resolved::Single(format!("{} 1 {}", g, b)),
            (None, Some(s), Some(b)) => Resolved::Single(format!("0 {} {}", s, b)),
            (Some(g), None, None) => Resolved::Single(g.to_string()),
            (None, Some(s), None) => Resolved::Single(format!("0 {}", s)),
            (None, None, Some(b)) => Resolved::Single(format!("0 1 {}", b)),
        }
    }
}

fn nonempty(s: Option<&str>) -> Option<&str> {
    s.filter(|s| !s.is_empty())
}

fn to_str(v: &Value) -> Option<&str> {
    match v {
        Value::String(s) => Some(s.as_str()),
        Value::Number(_) => None, // convert via format below
        _ => None,
    }
}

fn basis_to_str(v: &Value) -> Option<String> {
    if let Some(obj) = v.as_object() {
        let size = obj.get("size").and_then(|s| s.as_f64()).map(|n| {
            if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{}", n) }
        }).unwrap_or_default();
        let unit = obj.get("unit").and_then(|u| u.as_str()).unwrap_or("");
        return Some(format!("{}{}", size, unit));
    }
    v.as_str().map(|s| s.to_string())
        .or_else(|| v.as_f64().map(|n| {
            if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{}", n) }
        }))
}
