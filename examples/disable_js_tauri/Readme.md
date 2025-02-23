## Example for tauri `WebviewBuilder.disable_javascript`

PR: (TODO)

## Usage

```
cargo run
```

## What?

```rs
tauri::webview::WebviewBuilder::new("main1", WebviewUrl::App(Default::default()))
    .disable_javascript() // <- this line
    .auto_resize()
```

you can comment out this line to test the option.
