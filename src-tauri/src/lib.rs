use chrono::Local;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use showfile;
use std::{fs, io, path::Path, string::String};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager,
};
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn copy_dir(source_dir: String, target_dir: String) {
    // 检查目录是否为空
    if source_dir.is_empty() || target_dir.is_empty() {
        return;
    }
    // 检查源目录是否存在
    if !&Path::new(&source_dir).exists() {
        return;
    }
    // 创建目标目录
    fs::create_dir_all(&target_dir).expect("create target dir failed");
    // 生成时间目录
    let time_dir = gen_time_dir();
    let target_dir = target_dir.as_str().to_owned() + time_dir.as_str();
    // 递归复制目录和文件
    copy_dir_recursive(&Path::new(&source_dir), &Path::new(&target_dir)).expect("copy dir failed");
}

#[tauri::command]
fn save_json_string(json_data: &str, file_path: &str) -> Result<(), String> {
    // 将 JSON 字符串写入文件
    let res = fs::write(file_path, json_data);

    if res.is_err() {
        Err(io::Error::new(io::ErrorKind::NotFound, "save_json_string file not found").to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
fn load_json_string(file_path: &str) -> Result<String, String> {
    // 从文件读取 JSON 字符串
    let res = fs::read_to_string(file_path);

    if res.is_err() {
        Err(io::Error::new(io::ErrorKind::NotFound, "load_json_string file not found").to_string())
    } else {
        Ok(res.unwrap())
    }
}

fn gen_time_dir() -> String {
    let local = Local::now();
    return local.format("/%Y-%m-%d_%H-%M-%S").to_string();
}

fn copy_dir_recursive(from: &Path, to: &Path) -> Result<(), io::Error> {
    if !from.exists() {
        return Ok(());
    }
    if from.is_dir() {
        fs::create_dir_all(to)?;

        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let source_path = entry.path();
            let target_path = to.join(source_path.file_name().unwrap().to_str().unwrap());

            if source_path.is_dir() {
                copy_dir_recursive(&source_path, &target_path)?;
            } else {
                fs::copy(&source_path, &target_path)?;
            }
        }
    } else {
        fs::copy(from, to)?;
    }

    Ok(())
}

#[tauri::command]
fn show_in_folder(path: String) {
    showfile::show_path_in_file_manager(path);
}

pub fn run() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![show_in_folder, copy_dir, save_json_string, load_json_string])
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let cust_menu =
                MenuItemBuilder::with_id("Online Documentation", "Online Documentation")
                    .build(app)?;
            let sub_menu = SubmenuBuilder::new(app, "Help").item(&cust_menu).build()?;
            let menu = MenuBuilder::new(app).item(&sub_menu).build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == cust_menu.id() {
                    let _ = app
                        .shell()
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
