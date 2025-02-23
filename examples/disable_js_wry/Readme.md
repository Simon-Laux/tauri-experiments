## Example for wry `WebViewBuilder.with_javascript_disabled`

PR: https://github.com/tauri-apps/wry/pull/1496

## Usage

```
cargo run
```

## What?

```rs
let builder = WebViewBuilder::new()
        .with_javascript_disabled()  // <- this line
        .with_custom_protocol("wry", handler)
```

you can comment out this line to test the option.
