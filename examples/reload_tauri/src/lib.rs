// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{sync::Arc, time::Duration};

use tauri::{async_runtime::spawn, WebviewUrl, WindowEvent};
use tokio::time::sleep;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window =
                tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                    .build()?;

            let window_arc = Arc::new(window);
            let window = window_arc.clone();

            window_arc.on_window_event(move |ev| {
                if let WindowEvent::Moved { .. } = ev {
                    let _ = window.reload();
                }
                if let WindowEvent::Resized(_) = ev {
                    let _ = window.reload();
                }
            });

            let window = window_arc.clone();
            let _ = spawn(async move {
                loop {
                    sleep(Duration::from_secs(2)).await;
                    let _ = window.reload();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
