# elementor-style-renderer-rs

Rust port of [Elementor's PHP `styles-renderer` / `props-resolver` modules](https://github.com/elementor/elementor) from the atomic widgets architecture.

Converts Elementor atomic widget `PropValue` trees into CSS strings. Part of the [`elementor-to-blocks`](https://github.com/bhubbard/elementor-to-blocks) WordPress content migration toolchain.

> **Not affiliated with or endorsed by Elementor Ltd.**

## Installation

```toml
[dependencies]
elementor-style-renderer-rs = "0.1"
```

## Usage

```rust
use elementor_style_renderer_rs::{StylesRenderer, PropsResolver};

let renderer = StylesRenderer::new();
let css = renderer.render(&prop_value_tree, ".my-selector");
// => ".my-selector { color: #ff0000; padding: 10px 20px; }"
```

## Related Crates

| Crate | Purpose |
|---|---|
| [`elementor-style-converter-rs`](https://crates.io/crates/elementor-style-converter-rs) | Converts CSS strings → typed PropValue trees (the inverse) |
| [`wp-style-engine-rs`](https://crates.io/crates/wp-style-engine-rs) | Compiles Gutenberg block style objects to CSS |

## License

MIT — see [LICENSE](./LICENSE).
