use std::fs::File;
use app_dirs2::{get_app_root, AppDataType, AppInfo};
use notify_rust::Notification;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::time::Duration;
use enigo::Key;
use single_instance::SingleInstance;

const APP_INFO: AppInfo = AppInfo {
    name: "Natro Macro",
    author: "Spyied",
};

static APPDATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
    get_app_root(AppDataType::UserData, &APP_INFO)
        .expect("Should be able to get app data directory")
});

const FWD_KEY: Key = Key::Unicode('w');
const LEFT_KEY: Key = Key::Unicode('a');
const BACK_KEY: Key = Key::Unicode('s');
const RIGHT_KEY: Key = Key::Unicode('d');
const ROT_LEFT: Key = Key::Unicode(',');
const ROT_RIGHT: Key = Key::Unicode('.');
const ROT_UP: Key = Key::PageUp;
const ROT_DOWN: Key = Key::PageDown;
const ZOOM_IN: Key = Key::Unicode('i');
const ZOOM_OUT: Key = Key::Unicode('o');
const SC_E: Key = Key::Unicode('e');
const SC_R: Key = Key::Unicode('r');
const SC_L: Key = Key::Unicode('l');
const SC_ESC: Key = Key::Escape;
const SC_ENTER: Key = Key::Return;
const SC_LSHIFT: Key = Key::LShift;
const SC_SPACE: Key = Key::Space;
const SC_1: Key = Key::Unicode('1');

fn main() {
    let instance = SingleInstance::new("natro-macro").unwrap();
    if !instance.is_single() {
        Notification::new()
            .summary("Natro Macro is already running")
            .body("Natro Macro is already running please close the other instance")
            .show().expect("Should be able to notify");
        return;
    }

    std::thread::sleep(Duration::from_secs(u64::MAX));
}

