use crate::transformer::{Transformer, str_field};
use crate::types::{Resolved, TransformerContext};
use serde_json::Value;
use std::collections::HashMap;

const DEFAULT_IMAGE: &str = "none";
const DEFAULT_REPEAT: &str = "repeat";
const DEFAULT_ATTACHMENT: &str = "scroll";
const DEFAULT_SIZE: &str = "auto auto";
const DEFAULT_POSITION: &str = "0% 0%";

// ─── Background (top-level) ───────────────────────────────────────────────────

pub struct BackgroundTransformer;

impl Transformer for BackgroundTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let mut props: HashMap<String, String> = HashMap::new();

        // overlay props first
        if let Some(overlay) = value.get("background-overlay").and_then(|v| v.as_object()) {
            for (k, v) in overlay {
                if let Some(s) = v.as_str() {
                    props.insert(k.clone(), s.to_string());
                }
            }
        }

        if let Some(color) = str_field(value, "color") {
            props.insert("background-color".to_string(), color.to_string());
        }
        if let Some(clip) = str_field(value, "clip") {
            props.insert("background-clip".to_string(), clip.to_string());
        }

        if props.is_empty() {
            Resolved::None
        } else {
            Resolved::Multi(props)
        }
    }
}

// ─── Background overlay (array of layer objects) ──────────────────────────────

pub struct BackgroundOverlayTransformer;

impl Transformer for BackgroundOverlayTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let arr = match value.as_array() {
            Some(a) => a,
            None => return Resolved::None,
        };

        let normalized: Vec<&Value> = arr
            .iter()
            .filter(|v| v.get("src").and_then(|s| s.as_str()).is_some())
            .collect();

        if normalized.is_empty() {
            return Resolved::None;
        }

        let mut props = HashMap::new();
        props.insert(
            "background-image".to_string(),
            get_layer_values(&normalized, "src", DEFAULT_IMAGE, true),
        );
        props.insert(
            "background-repeat".to_string(),
            get_layer_values(&normalized, "repeat", DEFAULT_REPEAT, false),
        );
        props.insert(
            "background-attachment".to_string(),
            get_layer_values(&normalized, "attachment", DEFAULT_ATTACHMENT, false),
        );
        props.insert(
            "background-size".to_string(),
            get_layer_values(&normalized, "size", DEFAULT_SIZE, false),
        );
        props.insert(
            "background-position".to_string(),
            get_layer_values(&normalized, "position", DEFAULT_POSITION, false),
        );

        Resolved::Multi(props)
    }
}

fn get_layer_values(layers: &[&Value], prop: &str, default: &str, prevent_unification: bool) -> String {
    let values: Vec<String> = layers
        .iter()
        .map(|layer| {
            layer.get(prop)
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or(default)
                .to_string()
        })
        .collect();

    if !prevent_unification {
        let unique: std::collections::HashSet<&String> = values.iter().collect();
        if unique.len() == 1 {
            return values[0].clone();
        }
    }

    values.join(",")
}

// ─── Background color overlay ─────────────────────────────────────────────────

pub struct BackgroundColorOverlayTransformer;

impl Transformer for BackgroundColorOverlayTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        match str_field(value, "color").filter(|s| !s.is_empty()) {
            Some(c) => Resolved::Single(format!("linear-gradient({}, {})", c, c)),
            None => Resolved::None,
        }
    }
}

// ─── Background gradient overlay ──────────────────────────────────────────────

pub struct BackgroundGradientOverlayTransformer;

impl Transformer for BackgroundGradientOverlayTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let type_ = str_field(value, "type").unwrap_or("linear");
        let angle = value.get("angle").and_then(|v| v.as_f64()).unwrap_or(180.0) as i64;
        let positions = str_field(value, "positions").unwrap_or("center");
        let stops = str_field(value, "stops").unwrap_or("");

        let css = if type_ == "radial" {
            format!("radial-gradient(circle at {}, {})", positions, stops)
        } else {
            format!("linear-gradient({}deg, {})", angle, stops)
        };

        Resolved::Single(css)
    }
}

// ─── Background image overlay ─────────────────────────────────────────────────

pub struct BackgroundImageOverlayTransformer;

impl Transformer for BackgroundImageOverlayTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let image_src = value
            .get("image")
            .and_then(|img| img.get("src"))
            .and_then(|s| s.as_str());

        let image_src = match image_src {
            Some(s) => s,
            None => return Resolved::Single(String::new()),
        };

        // Returns a map-like structure; stored as a JSON object in practice.
        // For our pipeline we emit the fields as a multi-prop string map.
        let mut map = HashMap::new();
        map.insert("src".to_string(), format!("url(\"{}\")", image_src));
        if let Some(r) = str_field(value, "repeat") {
            map.insert("repeat".to_string(), r.to_string());
        }
        if let Some(a) = str_field(value, "attachment") {
            map.insert("attachment".to_string(), a.to_string());
        }
        if let Some(s) = str_field(value, "size") {
            map.insert("size".to_string(), s.to_string());
        }
        if let Some(p) = str_field(value, "position") {
            map.insert("position".to_string(), p.to_string());
        }

        Resolved::Multi(map)
    }
}

// ─── Background image overlay size scale ──────────────────────────────────────

pub struct BackgroundImageOverlaySizeScaleTransformer;

impl Transformer for BackgroundImageOverlaySizeScaleTransformer {
    fn transform(&self, value: &Value, _ctx: &TransformerContext) -> Resolved {
        let default = "auto";
        let width = str_field(value, "width").unwrap_or(default);
        let height = str_field(value, "height").unwrap_or(default);
        Resolved::Single(format!("{} {}", width, height))
    }
}
