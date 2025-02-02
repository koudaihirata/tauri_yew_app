use tauri::LogicalPosition;
use tauri::Manager;
use tauri::Window;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            position_window_to_top_right(&window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 画面の右上にウィンドウを配置する関数
fn position_window_to_top_right(window: &tauri::WebviewWindow) {
    if let Ok(monitor) = window.primary_monitor() {
        if let Some(monitor) = monitor {
            let size = monitor.size();
            let window_size = window.outer_size().unwrap();

            // 画面の右上に位置を設定 (マージン10px)
            let x = size.width - window_size.width - 10;
            let y = 10;

            window.set_position(LogicalPosition::new(x, y)).unwrap();
        }
    }
}
