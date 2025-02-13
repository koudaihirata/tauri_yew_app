use tauri::{Listener, LogicalPosition, PhysicalSize};
use tauri::{Manager, WindowBuilder};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // メインウィンドウの取得（モニター情報取得のために利用）
            let main_window = app.get_webview_window("main").unwrap();

            // ※ メインウィンドウを右上に配置
            position_window_to_top_right(&main_window);

            // blackout_window の作成
            // fullscreen() を使わず、visible(false) で非表示状態から開始
            let blackout_window = WindowBuilder::new(app, "blackout_window")
                .title("Blackout")
                .resizable(false)
                .decorations(false)
                .always_on_top(false)
                .visible(false)
                .build()
                .unwrap();

            // メインウィンドウからプライマリモニターのサイズを取得し、
            // blackout_window のサイズを設定することで画面全体（100%）に合わせる
            if let Ok(monitor) = main_window.primary_monitor() {
                if let Some(monitor) = monitor {
                    let monitor_size = monitor.size();
                    blackout_window
                        .set_size(PhysicalSize::new(monitor_size.width, monitor_size.height))
                        .unwrap();
                    // 画面全体を覆うため左上に配置
                    blackout_window
                        .set_position(LogicalPosition::new(0, 0))
                        .unwrap();
                }
            }

            // フロントエンドからウィンドウの表示・非表示を切り替えるリスナーを追加
            app.handle().listen("toggle_blackout", move |_| {
                if blackout_window.is_visible().unwrap_or(false) {
                    blackout_window.hide().unwrap();
                } else {
                    blackout_window.show().unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ※ 以下の関数は利用していないので、必要に応じて削除または使用してください。
fn position_window_to_top_right(window: &tauri::WebviewWindow) {
    if let Ok(monitor) = window.primary_monitor() {
        if let Some(monitor) = monitor {
            let size = monitor.size();
            let window_size = window.outer_size().unwrap();

            // 右上にウィンドウを配置（マージン10px）
            let x = size.width - window_size.width - 10;
            let y = 10;
            window.set_position(LogicalPosition::new(x, y)).unwrap();
        }
    }
}
