#[cfg(test)]
mod tests {
    use crate::{build_registry, Breakpoint, BreakpointDirection, StyleDefinition, StylesRenderer};
    use serde_json::json;
    use std::collections::HashMap;

    fn renderer<'a>(
        reg: &'a crate::transformer::TransformerRegistry,
        bp: &'a HashMap<String, Breakpoint>,
    ) -> StylesRenderer<'a> {
        StylesRenderer::new(reg, bp)
    }

    fn no_breakpoints() -> HashMap<String, Breakpoint> {
        HashMap::new()
    }

    fn parse_styles(json: serde_json::Value) -> Vec<StyleDefinition> {
        serde_json::from_value(json).unwrap()
    }

    // ─── Color passthrough ────────────────────────────────────────────────────

    #[test]
    fn renders_color_prop() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let r = renderer(&reg, &bp);

        let styles = parse_styles(json!([{
            "id": "my-class",
            "type": "class",
            "variants": [{"props": {"color": {"$$type": "color", "value": "red"}}, "meta": {}}]
        }]));

        let css = r.render(&styles);
        assert!(css.contains("color:red;"), "got: {}", css);
    }

    // ─── Size transformer ─────────────────────────────────────────────────────

    #[test]
    fn renders_size_px() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "h",
            "type": "class",
            "variants": [{"props": {"font-size": {"$$type": "size", "value": {"size": 16, "unit": "px"}}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("font-size:16px;"), "got: {}", css);
    }

    #[test]
    fn renders_size_auto() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "h",
            "type": "class",
            "variants": [{"props": {"width": {"$$type": "size", "value": {"size": null, "unit": "auto"}}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("width:auto;"), "got: {}", css);
    }

    #[test]
    fn renders_size_custom() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "h",
            "type": "class",
            "variants": [{"props": {"width": {"$$type": "size", "value": {"size": "calc(100% - 20px)", "unit": "custom"}}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("width:calc(100% - 20px);"), "got: {}", css);
    }

    // ─── Selector prefix ──────────────────────────────────────────────────────

    #[test]
    fn uses_selector_prefix() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "my-class",
            "type": "class",
            "variants": [{"props": {"color": {"$$type": "color", "value": "blue"}}, "meta": {}}]
        }]));
        let css = StylesRenderer::new(&reg, &bp)
            .with_selector_prefix(".e-widget")
            .render(&styles);
        assert!(css.starts_with(".e-widget .my-class{"), "got: {}", css);
    }

    #[test]
    fn uses_css_name_when_provided() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "abc123",
            "type": "class",
            "cssName": "my-heading",
            "variants": [{"props": {"color": {"$$type": "color", "value": "green"}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains(".my-heading{"), "got: {}", css);
    }

    // ─── State selectors ──────────────────────────────────────────────────────

    #[test]
    fn renders_hover_state() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "my-class",
            "type": "class",
            "variants": [{
                "props": {"color": {"$$type": "color", "value": "pink"}},
                "meta": {"state": "hover"}
            }]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        // hover also implies focus-visible; both selectors share one rule block
        assert!(css.contains(":hover,"), "got: {}", css);
        assert!(css.contains(":focus-visible{"), "got: {}", css);
    }

    // ─── Breakpoint media query ───────────────────────────────────────────────

    #[test]
    fn wraps_with_max_width_media_query() {
        let reg = build_registry();
        let mut bp = HashMap::new();
        bp.insert("mobile".to_string(), Breakpoint {
            direction: BreakpointDirection::Max,
            value: 767,
            is_enabled: true,
        });

        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [{
                "props": {"color": {"$$type": "color", "value": "red"}},
                "meta": {"breakpoint": "mobile"}
            }]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.starts_with("@media(max-width:767px){"), "got: {}", css);
    }

    #[test]
    fn disabled_breakpoint_returns_empty() {
        let reg = build_registry();
        let mut bp = HashMap::new();
        bp.insert("tablet".to_string(), Breakpoint {
            direction: BreakpointDirection::Min,
            value: 768,
            is_enabled: false,
        });

        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [{
                "props": {"color": {"$$type": "color", "value": "red"}},
                "meta": {"breakpoint": "tablet"}
            }]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.is_empty(), "got: {}", css);
    }

    // ─── Invalid style type silently skipped ──────────────────────────────────

    #[test]
    fn skips_non_class_types() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "foo",
            "type": "id",  // not "class"
            "variants": [{"props": {"color": {"$$type": "color", "value": "red"}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.is_empty());
    }

    // ─── Flex transformer ─────────────────────────────────────────────────────

    #[test]
    fn renders_flex_three_values() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [{"props": {"flex": {"$$type": "flex", "value": {
                "flexGrow": "1",
                "flexShrink": "2",
                "flexBasis": {"size": 50, "unit": "px"}
            }}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("flex:1 2 50px;"), "got: {}", css);
    }

    // ─── Font family quoting ──────────────────────────────────────────────────

    #[test]
    fn quotes_font_family() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [{"props": {"font-family": {"$$type": "font-family", "value": "Inter"}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("font-family:\"Inter\";"), "got: {}", css);
    }

    // ─── Multiple variants ────────────────────────────────────────────────────

    #[test]
    fn renders_multiple_variants() {
        let reg = build_registry();
        let mut bp = HashMap::new();
        bp.insert("mobile".to_string(), Breakpoint {
            direction: BreakpointDirection::Max,
            value: 767,
            is_enabled: true,
        });

        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [
                {"props": {"color": {"$$type": "color", "value": "red"}}, "meta": {}},
                {"props": {"color": {"$$type": "color", "value": "blue"}}, "meta": {"breakpoint": "mobile"}}
            ]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("color:red;"));
        assert!(css.contains("@media(max-width:767px)"));
        assert!(css.contains("color:blue;"));
    }

    // ─── Disabled prop skipped ────────────────────────────────────────────────

    #[test]
    fn skips_disabled_prop() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [{"props": {"color": {"$$type": "color", "value": "red", "disabled": true}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(!css.contains("color:"), "disabled prop should not appear; got: {}", css);
    }

    // ─── Grid track size (fr) ─────────────────────────────────────────────────

    #[test]
    fn renders_grid_track_fr() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [{"props": {"grid-template-columns": {"$$type": "grid-track-size", "value": {"size": 3, "unit": "fr"}}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("grid-template-columns:repeat(3, 1fr);"), "got: {}", css);
    }

    // ─── Shadow transformer ───────────────────────────────────────────────────

    #[test]
    fn renders_shadow() {
        let reg = build_registry();
        let bp = no_breakpoints();
        let styles = parse_styles(json!([{
            "id": "el",
            "type": "class",
            "variants": [{"props": {"box-shadow": {"$$type": "shadow", "value": {
                "hOffset": "2px",
                "vOffset": "4px",
                "blur": "8px",
                "spread": "0px",
                "color": "#000"
            }}}, "meta": {}}]
        }]));
        let css = renderer(&reg, &bp).render(&styles);
        assert!(css.contains("box-shadow:2px 4px 8px 0px #000;"), "got: {}", css);
    }
}
