use std::sync::Arc;

use objc2::MainThreadMarker;
use objc2_app_kit::NSApp;
use objc2_foundation::NSUUID;
use objc2_web_kit::WKWebsiteDataStore;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tokio::spawn;
use wry::{WebViewBuilder, WebViewBuilderExtDarwin as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mtm = MainThreadMarker::new().unwrap();
    // let identifier = NSUUID::from_bytes([0; 16]);
    // let bytes = NSUUID::new().as_bytes();
    // println!("{bytes:?}");
    let identifier = NSUUID::from_bytes([
        119, 101, 98, 120, 100, 99, 95, 95, 0, 0, 0, 1, 0, 0, 52, 143,
    ]);
    unsafe {
        WKWebsiteDataStore::dataStoreForIdentifier(&identifier, mtm);
    }
    let identifier =
        NSUUID::from_bytes([1, 101, 98, 120, 100, 99, 95, 95, 0, 0, 0, 1, 0, 0, 52, 143]);
    unsafe {
        WKWebsiteDataStore::dataStoreForIdentifier(&identifier, mtm);
    }

    wry::fetch_all_data_store_identifiers(move |ids| {
        println!("- {:?}", ids);
        wry::remove_data_store(ids.get(0).unwrap(), |result| {
            println!("deletion result - {:?}", result);
            wry::fetch_all_data_store_identifiers(|ids| {
                println!("after: {:?}", ids);
            })
            .unwrap();
        })
        .unwrap();
    })?;

    // let id = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193, 0, 0, 0, 0, 0];

    // wry::remove_data_store(&id, |result| {
    //     println!("deletion result - {:?}", result);
    // })?;

    let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();
    // wry::fetch_all_data_store_identifiers(move |ids| {
    //     println!("-- {:?}", ids);
    // })?;
    // let builder = WebViewBuilder::new()
    //     .with_data_store_identifier([12, 101, 98, 120, 100, 99, 95, 95, 0, 0, 0, 1, 0, 0, 52, 143])
    //     .build(&window)?;
    // wry::fetch_all_data_store_identifiers(move |ids| {
    //     println!("--- {:?}", ids);
    // })?;

    // let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<[u8; 16]>>(1);
    // let tx = Arc::new(tx);
    // println!("1");
    // wry::fetch_all_data_store_identifiers(move |ids| {
    //     println!("2 {:?}", ids);
    //     let tx = tx.clone();
    //     spawn(async move { tx.send(ids).await });
    // })?;
    // println!("3");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    })
}
