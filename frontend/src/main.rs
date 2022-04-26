use yew::prelude::*;

#[function_component(App)]
fn view() -> Html {
    let data = use_state(|| String::new());

    use_effect_with_deps(
        {
            let data = data.clone();
            move |_| {
                let data = data.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data = reqwasm::http::Request::get("/api/data")
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    data.set(fetched_data);
                });
                || ()
            }
        },
        (),
    );

    html! {
        <p class={classes!("bg-red-100")}>{&*data}</p>
    }
}

fn main() {
    yew::start_app::<App>();
}
