use gloo_net::http::Request;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[function_component]
fn GetRequest() -> Html {
    let text = use_state(|| String::new());
    let api_route = use_state(|| String::new());
    let onclick = {
        let text_a = text.clone();
        let api_route_a = api_route.clone();
        move |_| {
            let text = text_a.clone();
            let api_route = api_route_a.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req = Request::get(&api_route)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                text.set(req);
            });
        }
    };

    let onchange = 
        Callback::from(move |e: Event| {
        let api_route = api_route.clone();
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        api_route.set(input.value()); 
    });

    html! {
        <div>
            <h3>{ "Enter get request: " } </h3>
            <input {onchange} />
            <button {onclick}> { "Click Me" } </button>
            <p>{ &*text }</p>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <GetRequest />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
