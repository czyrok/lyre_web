# Prerequis

```shell
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

# Issues

## Wasm-binden version issue

The key correspondence here is between the version of `cargo-leptos` you have installed (which includes a `wasm-bindgen-cli-support` dependency) and the version of `wasm-bindgen` that is being pulled into the crate being compiled.

**Solutions:**

1. Update cargo-leptos

```shell
cargo install cargo-leptos --version '=0.2.27' --locked
```

2. Pin the wasm-bindgen dependency in your crate to the one being used by the cargo-leptos version you have installed

```toml
wasm-bindgen = "=0.2.100"
```

## Trait bounds not satisfied

```
the method `get`exists for struct`Resource<Result<Vec<Post>, ServerFnError>>`, but its trait bounds were not satisfied
```

=> You need to add the `Clone` trait on custom error used in the server function error and on the type of the server function's returned data.

## Hydration error

```
A hydration error occurred while trying to hydrate an element defined at src/project/views/project_details.rs:65:10.

The framework expected a marker node, but found this instead:  <h1>​Plantrakt​</h1>​

The hydration mismatch may have occurred slightly earlier, but this is the first time the framework found a node of an unexpected type.
```

=> Check the HTML static files for errors.

## StreamBuilder issue

```
error[E0599]: no method named `push_async_out_of_order_with_nonce` found for mutable reference `&mut StreamBuilder` in the current scope
```

=> It was a field where I didn't set the `Option` as the type and where I sed `#[serde(skip_serializing)]`. So, the error was from the deserializer that telled me that the field is missing. It can be useful to check the error from the Leptos `Resource`.
