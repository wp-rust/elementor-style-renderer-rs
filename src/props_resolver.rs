/// PropValue → CSS string resolver — port of PHP `Render_Props_Resolver`.
///
/// Pipeline per prop:
/// 1. Check if value is transformable (`$$type` + `value` present)
/// 2. Recursively resolve object/array children first
/// 3. Dispatch to the transformer matching `$$type`
/// 4. If the result is itself transformable, recurse (up to DEPTH_LIMIT)
/// 5. Handle `$$multi-props` wrapper → expand into multiple CSS declarations

use crate::transformer::TransformerRegistry;
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;
use std::collections::HashMap;

const DEPTH_LIMIT: u32 = 3;

pub struct PropsResolver<'a> {
    registry: &'a TransformerRegistry,
}

impl<'a> PropsResolver<'a> {
    pub fn new(registry: &'a TransformerRegistry) -> Self {
        Self { registry }
    }

    /// Resolve all props in `props` against the registered transformers.
    /// Returns a map of CSS `property → value` strings.
    pub fn resolve(&self, props: &HashMap<String, Value>) -> HashMap<String, String> {
        let mut out: HashMap<String, String> = HashMap::new();

        for (key, raw) in props {
            match self.resolve_item(raw, key, 0) {
                Resolved::Single(s) => {
                    out.insert(key.clone(), s);
                }
                Resolved::Multi(m) => {
                    out.extend(m);
                }
                Resolved::None => {}
            }
        }

        out
    }

    fn resolve_item(&self, value: &Value, key: &str, depth: u32) -> Resolved {
        if value.is_null() {
            return Resolved::None;
        }

        if !is_transformable(value) {
            // Not tagged — pass through as raw string if possible
            return match value.as_str() {
                Some(s) => Resolved::Single(s.to_string()),
                None => Resolved::None,
            };
        }

        if depth >= DEPTH_LIMIT {
            return Resolved::None;
        }

        // Check disabled flag
        if value.get("disabled").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Resolved::None;
        }

        let type_key = value["$$type"].as_str().unwrap_or("");
        let inner = &value["value"];

        // Recurse into object-typed values
        let resolved_inner = self.resolve_inner(type_key, inner, key);

        let transformer = match self.registry.get(type_key) {
            Some(t) => t,
            None => return Resolved::None,
        };

        let ctx = TransformerContext::new(key);
        transformer.transform(&resolved_inner, &ctx)
    }

    fn resolve_inner(&self, _type_key: &str, inner: &Value, key: &str) -> Value {
        match inner {
            Value::Object(obj) => {
                let mut resolved = serde_json::Map::new();
                for (k, v) in obj {
                    let r = self.resolve_item(v, k, 0);
                    resolved.insert(k.clone(), match r {
                        Resolved::Single(s) => Value::String(s),
                        _ => v.clone(),
                    });
                }
                Value::Object(resolved)
            }
            Value::Array(arr) => {
                // For array types, resolve each item individually
                let items: Vec<Value> = arr
                    .iter()
                    .filter_map(|item| {
                        if is_transformable(item) {
                            match self.resolve_item(item, key, 0) {
                                Resolved::Single(s) => Some(Value::String(s)),
                                _ => None,
                            }
                        } else {
                            Some(item.clone())
                        }
                    })
                    .collect();
                Value::Array(items)
            }
            _ => inner.clone(),
        }
    }
}

fn is_transformable(value: &Value) -> bool {
    value.get("$$type").and_then(|v| v.as_str()).is_some()
        && value.get("value").is_some()
}
