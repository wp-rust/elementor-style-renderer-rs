use crate::transformer::{Transformer, str_field};
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;

/// Array of filter function objects → `"blur(5px) brightness(1.2)"`.
pub struct FilterTransformer;

impl Transformer for FilterTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let arr = match value.as_array() {
            Some(a) => a,
            None => return Resolved::None,
        };

        let parts: Vec<String> = arr.iter().filter_map(|f| filter_to_str(f)).collect();

        if parts.is_empty() {
            Resolved::None
        } else {
            Resolved::Single(parts.join(" "))
        }
    }
}

fn filter_to_str(filter: &Value) -> Option<String> {
    let func = str_field(filter, "func")?;
    let args = filter.get("args")?;

    if func == "drop-shadow" {
        let x = str_field(args, "xAxis").unwrap_or("0px");
        let y = str_field(args, "yAxis").unwrap_or("0px");
        let blur = str_field(args, "blur").unwrap_or("10px");
        let color = str_field(args, "color").unwrap_or("transparent");
        return Some(format!("drop-shadow({} {} {} {})", x, y, blur, color));
    }

    let size = str_field(args, "size")?;
    Some(format!("{}({})", func, size))
}
