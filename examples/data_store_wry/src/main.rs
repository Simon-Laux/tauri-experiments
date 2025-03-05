// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::borrow::Cow;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilderExtDarwin;
use wry::{
    WebViewBuilder,
    http::{Response, header::CONTENT_TYPE},
};

#[cfg(target_os = "macos")]
fn main() -> wry::Result<()> {
    use std::thread;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let builder = WebViewBuilder::new()
        .with_custom_protocol("wry".into(), move |_webview_id, _request| {
            let response = include_str!("main.html");

            Response::builder()
                .header(CONTENT_TYPE, "text/html")
                .body(Cow::from(response.as_bytes()))
                .unwrap()
        })
        .with_data_store_identifier([
            119, 101, 98, 120, 100, 99, 95, 95, 0, 0, 0, 1, 0, 0, 52, 143,
        ])
        // tell the webview to load the custom protocol
        .with_url("wry://localhost");

    let _webview = builder.build(&window)?;

    println!("{:?}", thread::current().id());
    println!("1");
    wry::fetch_all_data_store_identifiers(move |ids| {
        println!("{:?}", thread::current().id());
        println!("2 {ids:?}");
    })?;
    println!("3");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
    // Ok(())
}
