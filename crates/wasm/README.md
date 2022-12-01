# Demo of `cairo-rs` on WebAssembly

While cairo-rs is compatible with WebAssembly, it doesn't implement any bindings
to it. Instead, create a new WebAssembly crate with cairo-rs as a dependency and
implement the required functionality there.

Since mimalloc is not automatically compilable to WebAssembly, the cairo-rs
dependency should disable the default features, which will in turn disable
mimalloc.

WebAssembly doesn't support filesystem access unless building with WASI support,
therefore cairo_run may not work as is. Running programs requires manual
program, vm and runner initialization.

A working example is provided in this repository.

**Building**

```sh
# The web target generates a JavaScript module that is directly loadable by the
# browser.
wasm-pack build --target=web
```

**Running**

```sh
# Running from file:// will result in a CORS error.
wasm-pack build --target=web
```
