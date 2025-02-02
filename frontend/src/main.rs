use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <h1>{ "Yew + Tauri App" }</h1>
            <p>{ format!("Count: {}", *counter) }</p>
            <button {onclick}>{ "Increment" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
