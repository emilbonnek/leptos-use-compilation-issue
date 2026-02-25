//! Minimal reproduction of leptos-use 0.18.1 compilation errors with wasm-bindgen 0.2.113
//!
//! This demonstrates the compilation failures that occur when using leptos-use 0.18.1
//! with the latest wasm-bindgen 0.2.113 / web-sys 0.3.90.
//!
//! ## Errors
//!
//! 1. **Geolocation API**: `watch_position_with_error_callback_and_options()` returns
//!    `Result<i32, JsValue>` but leptos-use expects `i32`
//!
//! 2. **Scroll API**: Scroll properties return `f64` but leptos-use expects `i32`
//!
//! 3. **Mouse Events**: Coordinate methods have type mismatches
//!
//! ## To reproduce
//!
//! ```bash
//! cargo check --target wasm32-unknown-unknown
//! ```

#[cfg(target_arch = "wasm32")]
pub fn use_features() {
    use leptos::prelude::*;
    use leptos_use::{use_geolocation, use_mouse, use_scroll};

    // This will fail to compile due to Geolocation API changes
    let _geolocation = use_geolocation();

    // This will fail to compile due to Mouse event type changes
    let _mouse = use_mouse();

    // This will fail to compile due to Scroll API type changes
    let _scroll = use_scroll(document().body());
}
