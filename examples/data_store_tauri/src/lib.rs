use std::str::FromStr;

use tauri::{AppHandle, Url, WebviewUrl};
use uuid::Uuid;

#[tauri::command]
async fn new_window(app: AppHandle) -> Result<(), String> {
    let id = Uuid::new_v4().to_bytes_le();
    let window = tauri::WebviewWindowBuilder::new(
        &app,
        format!("win{}", Uuid::new_v4().to_string()),
        WebviewUrl::External(Url::from_str("about:blank").unwrap()),
    )
    .data_store_identifier(id)
    .build()
    .map_err(|err| format!("{:?}", err))?;
    window.show().map_err(|err| format!("{:?}", err))?;
    Ok(())
}

#[tauri::command]
async fn get_ids(app: AppHandle) -> Result<Vec<[u8; 16]>, String> {
    app.fetch_all_data_store_identifiers()
        .await
        .map_err(|err| format!("{:?}", err))
}

#[tauri::command]
async fn delete_id(app: AppHandle, uuid: [u8; 16]) -> Result<(), String> {
    Ok(app
        .remove_data_store(uuid)
        .await
        .map_err(|err| format!("{:?}", err))?)
}

#[cfg(target_vendor = "apple")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ids, delete_id, new_window])
        .setup(|app| {
            let _window =
                tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                    .build()?;

            Ok(())
        })
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
