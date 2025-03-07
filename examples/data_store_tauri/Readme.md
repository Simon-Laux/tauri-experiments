## Example for tauri `WebviewWindow.reload`

PR: https://github.com/tauri-apps/tauri/pull/12818

## Usage

```
cargo run
```

## What?

```rs
app.fetch_all_data_store_identifiers().await
app.remove_data_store(uuid).await
```

macOS/iOS only.
delete and list data_stores where browsing data like localstorage and cookies are stored.
