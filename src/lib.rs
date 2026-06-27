//! # elementor-style-renderer-rs
//!
//! Converts Elementor atomic widget PropValue trees into CSS strings.
//! Port of the PHP `styles-renderer` + `props-resolver` + style transformer modules.
//!
//! ## Usage
//!
//! ```rust
//! use elementor_style_renderer_rs::{StylesRenderer, StyleDefinition, Breakpoint, BreakpointDirection, build_registry};
//! use std::collections::HashMap;
//!
//! let registry = build_registry();
//! let breakpoints: HashMap<String, Breakpoint> = HashMap::new();
//! let renderer = StylesRenderer::new(&registry, &breakpoints);
//!
//! let styles: Vec<StyleDefinition> = serde_json::from_str(r#"[{
//!     "id": "my-heading",
//!     "type": "class",
//!     "variants": [{
//!         "props": {"color": {"$$type":"color","value":"red"}},
//!         "meta": {}
//!     }]
//! }]"#).unwrap();
//!
//! let css = renderer.render(&styles);
//! assert!(css.contains("color:red;"));
//! ```

pub mod props_resolver;
pub mod style_states;
pub mod styles_renderer;
pub mod transformer;
pub mod transformers;
pub mod types;
#[cfg(test)]
mod tests;

pub use styles_renderer::{StylesRenderer, build_registry};
pub use types::{Breakpoint, BreakpointDirection, StyleDefinition, StyleVariant, VariantMeta};
