/*
 * @Author: yiranzai wuqingdzx@gmail.com
 * @Date: 2024-07-12 23:26:28
 * @LastEditors: yiranzai wuqingdzx@gmail.com
 * @LastEditTime: 2024-07-13 01:09:28
 * @FilePath: /tauri-test/src-tauri/src/lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri_plugin_shell::ShellExt;

#[tauri::command]
fn backend_add(number: i32) -> i32 {
    // Note: these commands block the main thread and hang the UI until they return.
    // If you need to run a long-running task, use async command instead.
    println!("Backend was called with an argument: {}", number);
    number + 2
}

pub fn run() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![backend_add])
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let custMenu = MenuItemBuilder::with_id("Online Documentation", "Online Documentation")
                .build(app)?;
            let subMenu = SubmenuBuilder::new(app, "Help").item(&custMenu).build()?;
            let menu = MenuBuilder::new(app).item(&subMenu).build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == custMenu.id() {
                    let _ = app.shell()
                        .open("https://github.com/tauri-apps/tauri", None);
                }
            });

            #[cfg(debug_assertions)]
            {
                let main_window = app.get_webview_window("main").unwrap();
                main_window.open_devtools();
            }

            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
