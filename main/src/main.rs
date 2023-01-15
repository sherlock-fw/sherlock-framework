#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "linux"
)]

use sherlock_manager::SherlockManager;
use tauri::{Manager, Window};
// the payload type must implement `Serialize` and `Clone`.

// init a background process on the command, and emit periodic events only to the window that used the command
fn init_process(window: Window) {
    std::thread::spawn(move || {
        SherlockManager::init().attach(window).build().listen();
    });
}

fn main() {
    println!("start");
    tauri::Builder::default()
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            let main_window = app.get_window("main").unwrap();
            init_process(main_window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("after");
}
