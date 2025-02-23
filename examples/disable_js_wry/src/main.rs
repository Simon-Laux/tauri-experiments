// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::borrow::Cow;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    http::{header::CONTENT_TYPE, Response},
    WebViewBuilder,
};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let builder = WebViewBuilder::new()
        .with_javascript_disabled()
        .with_custom_protocol("wry".into(), move |_webview_id, _request| {
            let response = "<html>
<head></head>
<body>
<noscript>No Script</noscript>
<script>
var d = document.createElement('div');
d.textContent = 'JS is Active';
d.style = 'background-color:red;padding: 10px;margin: 10px';
document.body.appendChild(d);
</script>
<h5>Hello World</h5>
</body>
</html>";

            Response::builder()
                .header(CONTENT_TYPE, "text/html")
                .body(Cow::from(response.as_bytes()))
                .unwrap()
        })
        // tell the webview to load the custom protocol
        .with_url("wry://localhost");

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

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
}
