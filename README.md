# leptos-use 0.18.1 Compilation Issues with wasm-bindgen 0.2.113

Minimal reproduction of compilation errors in `leptos-use` v0.18.1 when used with `wasm-bindgen` v0.2.113 / `web-sys` v0.3.90.

## Problem

Both `leptos-use` v0.18.1 and `wasm-bindgen` v0.2.113 were released on February 24, 2026. The API changes in wasm-bindgen broke leptos-use compilation.

## Reproduction

```bash
cargo check --target wasm32-unknown-unknown
```

## Expected Errors

### 1. Geolocation API (`use_geolocation`)

**File:** `~/.cargo/registry/src/.../leptos-use-0.18.1/src/use_geolocation.rs:106`

**Error:**
```
error[E0308]: mismatched types
expected `i32`, found `Result<i32, JsValue>`
```

**Cause:** `Geolocation::watch_position_with_error_callback_and_options()` now returns `Result<i32, JsValue>` instead of `i32`.

### 2. Scroll API (`use_scroll`)

**File:** `~/.cargo/registry/src/.../leptos-use-0.18.1/src/use_scroll.rs:313-318`

**Errors:**
```
error[E0308]: mismatched types
expected `i32`, found floating-point number

error[E0277]: cannot add `f64` to `i32`
```

**Cause:** Scroll properties (`scroll_left`, `scroll_top`, etc.) now return `f64` instead of `i32`.

### 3. Mouse Events (`use_mouse`)

**File:** `~/.cargo/registry/src/.../leptos-use-0.18.1/src/use_mouse.rs:275-277`

**Errors:**
```
error[E0308]: mismatched types
expected `f64`, found `i32`
```

**Cause:** Mouse coordinate methods (`page_x()`, `page_y()`, `client_x()`, `client_y()`, `screen_x()`, `screen_y()`) have return type mismatches.

## Environment

- **Rust:** nightly-2026-02-16
- **wasm-bindgen:** 0.2.113
- **web-sys:** 0.3.90
- **leptos-use:** 0.18.1
- **leptos:** 0.8.16

## Related

- wasm-bindgen v0.2.113: https://github.com/rustwasm/wasm-bindgen/releases/tag/0.2.113
- leptos-use v0.18.1: https://github.com/Synphonyte/leptos-use/releases/tag/v0.18.1
