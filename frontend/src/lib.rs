#[cfg(not(target_arch = "wasm32"))]
mod tauri_sys {
    pub mod event {
        use serde::Serialize;
        pub fn emit(_event: &str, _payload: impl Serialize) -> Result<(), ()> {
            // 非 wasm32 環境向けスタブ
            Ok(())
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
use tauri_sys::event::emit;

#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "@tauri-apps/api/event")]
extern "C" {
    fn emit(event: &str, payload: JsValue) -> Promise;
}

#[cfg(target_arch = "wasm32")]
async fn tauri_emit(event: &str, payload: impl Serialize) {
    let js_payload = serde_wasm_bindgen::to_value(&payload).unwrap();
    let _ = JsFuture::from(emit(event, js_payload)).await;
}

use web_sys::window;
use web_sys::Document;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // デバッグログの出力（glooライブラリを利用）
    gloo::console::log!("App component rendered");
    let toggle_blackout = Callback::from(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            spawn_local(async {
                tauri_emit("toggle_blackout", ()).await;
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            spawn_local(async {
                let _ = emit("toggle_blackout", ());
            });
        }
    });

    html! {
        // デバッグ用に表示テキストを追加
        <div style="position: absolute; top: 10px; left: 10px; color: black;">
            <p>{"Hello, Yew App!"}</p>
            <div style="width: 40px; height: 40px; cursor: pointer;" onclick={toggle_blackout.clone()}>
                <svg width="40" height="40" viewBox="0 0 436 392" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M130.8 392V348.444H174.4V304.889H43.6C31.61 304.889 21.3458 300.624 12.8075 292.094C4.26917 283.565 0 273.311 0 261.333V43.5556C0 31.5778 4.26917 21.3241 12.8075 12.7944C21.3458 4.26481 31.61 0 43.6 0H392.4C404.39 0 414.654 4.26481 423.193 12.7944C431.731 21.3241 436 31.5778 436 43.5556V261.333C436 273.311 431.731 283.565 423.193 292.094C414.654 300.624 404.39 304.889 392.4 304.889H261.6V348.444H305.2V392H130.8ZM43.6 261.333H392.4V43.5556H43.6V261.333Z" fill="#5F6368"/>
                    <path d="M198.857 156.833L141.929 99.9049L160.905 80.9286L217.833 137.857L274.762 80.9286L293.738 99.9049L236.81 156.833L293.738 213.762L274.762 232.738L217.833 175.81L160.905 232.738L141.929 213.762L198.857 156.833Z" fill="#5F6368"/>
                </svg>
            </div>
        </div>
    }
}
fn main() {
    // 明示的に「app」要素にマウント
    let document: Document = window().unwrap().document().unwrap();
    let mount_node = document.get_element_by_id("app").unwrap();
    yew::Renderer::<App>::with_root(mount_node).render();
}
