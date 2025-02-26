// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, WebviewUrl, WindowEvent};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let width = 800.;
            let height = 600.;

            let window = tauri::window::WindowBuilder::new(app, "main")
                .inner_size(width, height)
                .build()?;

            let webview1 = window.add_child(
                tauri::webview::WebviewBuilder::new("main1", WebviewUrl::App(Default::default()))
                    .auto_resize(),
                LogicalPosition::new(0., 0.),
                LogicalSize::new(width, height),
            )?;

            window.on_window_event(move |ev| {
                if let WindowEvent::Moved { .. } = ev {
                    let _ = webview1.reload();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
