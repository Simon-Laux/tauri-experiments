use objc2::MainThreadMarker;
use objc2_foundation::NSUUID;
use objc2_web_kit::WKWebsiteDataStore;

fn main() {
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
}
