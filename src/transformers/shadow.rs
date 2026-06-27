use crate::transformer::{Transformer, str_field};
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;

/// `{hOffset, vOffset, blur, spread, color, position?}` → `"2px 4px 8px 0px #000 inset"`.
pub struct ShadowTransformer;

impl Transformer for ShadowTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let parts: Vec<&str> = [
            str_field(value, "hOffset"),
            str_field(value, "vOffset"),
            str_field(value, "blur"),
            str_field(value, "spread"),
            str_field(value, "color"),
            str_field(value, "position"),
        ]
        .into_iter()
        .flatten()
        .filter(|s| !s.is_empty())
        .collect();

        if parts.is_empty() {
            Resolved::None
        } else {
            Resolved::Single(parts.join(" "))
        }
    }
}
