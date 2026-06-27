/// CSS renderer — port of PHP `Styles_Renderer`.
///
/// Takes an array of `StyleDefinition` objects and renders them to a CSS string,
/// handling breakpoints (media queries) and pseudo-state selectors.

use crate::props_resolver::PropsResolver;
use crate::style_states;
use crate::transformer::TransformerRegistry;
use crate::types::{Breakpoint, BreakpointDirection, StyleDefinition, StyleVariant};
use std::collections::HashMap;

const DEFAULT_SELECTOR_PREFIX: &str = ".elementor";

pub struct StylesRenderer<'a> {
    registry: &'a TransformerRegistry,
    breakpoints: &'a HashMap<String, Breakpoint>,
    selector_prefix: String,
}

impl<'a> StylesRenderer<'a> {
    pub fn new(
        registry: &'a TransformerRegistry,
        breakpoints: &'a HashMap<String, Breakpoint>,
    ) -> Self {
        Self {
            registry,
            breakpoints,
            selector_prefix: DEFAULT_SELECTOR_PREFIX.to_string(),
        }
    }

    pub fn with_selector_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.selector_prefix = prefix.into();
        self
    }

    /// Render a slice of style definitions to a CSS string.
    pub fn render(&self, styles: &[StyleDefinition]) -> String {
        styles
            .iter()
            .map(|s| self.render_definition(s))
            .collect::<String>()
    }

    fn render_definition(&self, def: &StyleDefinition) -> String {
        let base = match self.base_selector(def) {
            Some(b) => b,
            None => return String::new(),
        };

        def.variants
            .iter()
            .filter_map(|v| self.render_variant(&base, v))
            .collect::<String>()
    }

    fn base_selector(&self, def: &StyleDefinition) -> Option<String> {
        if def.style_type != "class" || def.id.is_empty() {
            return None;
        }

        let name = def.css_name.as_deref().unwrap_or(&def.id);
        let prefix = &self.selector_prefix;

        if prefix.is_empty() {
            Some(format!(".{}", name))
        } else {
            Some(format!("{} .{}", prefix, name))
        }
    }

    fn render_variant(&self, base: &str, variant: &StyleVariant) -> Option<String> {
        let resolver = PropsResolver::new(self.registry);
        let resolved = resolver.resolve(&variant.props);

        let props_css: String = resolved
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| format!("{}:{};", k, v))
            .collect();

        let custom_css = variant
            .custom_css
            .as_ref()
            .and_then(|c| c.raw.as_deref())
            .map(|raw| {
                // raw is base64 in PHP (Utils::decode_string); in our pipeline
                // it may already be decoded or raw CSS — pass through as-is.
                raw.to_string()
            })
            .unwrap_or_default();

        if props_css.is_empty() && custom_css.is_empty() {
            return None;
        }

        let selector = match variant.meta.state.as_deref() {
            Some(state) => style_states::selector_with_state(base, state),
            None => base.to_string(),
        };

        let rule = format!("{}{{{}{}}}", selector, props_css, custom_css);

        match variant.meta.breakpoint.as_deref() {
            Some(bp_id) => Some(self.wrap_media_query(bp_id, rule)),
            None => Some(rule),
        }
    }

    fn wrap_media_query(&self, bp_id: &str, css: String) -> String {
        let bp = match self.breakpoints.get(bp_id) {
            Some(b) => b,
            None => return css,
        };

        if !bp.is_enabled {
            return String::new();
        }

        let bound = match bp.direction {
            BreakpointDirection::Min => "min-width",
            BreakpointDirection::Max => "max-width",
        };

        format!("@media({}:{}px){{{}}}", bound, bp.value, css)
    }
}

/// Build the standard transformer registry with all built-in transformers wired up.
pub fn build_registry() -> TransformerRegistry {
    use crate::transformers::*;

    let mut r = TransformerRegistry::default();

    // Size / grid
    r.register("size", SizeTransformer);
    r.register("grid-track-size", GridTrackSizeTransformer);

    // Layout
    r.register("flex", FlexTransformer);

    // Effects
    r.register("shadow", ShadowTransformer);
    r.register("filter", FilterTransformer);
    r.register("backdrop-filter", FilterTransformer);

    // Transform
    r.register("transform-functions", TransformFunctionsTransformer);
    r.register("transform-move", TransformMoveTransformer);
    r.register("transform-rotate", TransformRotateTransformer);
    r.register("transform-scale", TransformScaleTransformer);
    r.register("transform-skew", TransformSkewTransformer);
    r.register("transform-origin", TransformOriginTransformer);

    // Background
    r.register("background", BackgroundTransformer);
    r.register("background-overlay", BackgroundOverlayTransformer);
    r.register("background-color-overlay", BackgroundColorOverlayTransformer);
    r.register("background-gradient-overlay", BackgroundGradientOverlayTransformer);
    r.register("background-image-overlay", BackgroundImageOverlayTransformer);
    r.register("background-image-overlay-size-scale", BackgroundImageOverlaySizeScaleTransformer);

    // Misc
    r.register("span", SpanTransformer);
    r.register("font-family", FontFamilyTransformer);
    r.register("position", PositionTransformer);
    r.register("perspective-origin", PerspectiveOriginTransformer);
    r.register("color-stop", ColorStopTransformer);
    r.register("stroke", StrokeTransformer);
    r.register("transition", TransitionTransformer);

    // Simple passthrough: color, string, number — their "value" IS the CSS string
    r.register("color", PassthroughTransformer);
    r.register("string", PassthroughTransformer);
    r.register("number", PassthroughTransformer);

    r
}

/// Passthrough: `{"$$type": "color", "value": "red"}` → `"red"`.
struct PassthroughTransformer;

impl crate::transformer::Transformer for PassthroughTransformer {
    fn transform(&self, value: &serde_json::Value, _ctx: &crate::types::TransformerContext) -> crate::types::Resolved {
        match value {
            serde_json::Value::String(s) => crate::types::Resolved::Single(s.clone()),
            serde_json::Value::Number(n) => crate::types::Resolved::Single(n.to_string()),
            _ => crate::types::Resolved::None,
        }
    }
}
