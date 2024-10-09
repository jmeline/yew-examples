use yew::prelude::*;

#[function_component]
fn App() -> Html {

    let counter = use_state(|| 0);
    let on_increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let on_decrement = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };

    html! {
        <>
            <h1>{ "Counter" }</h1>
            <button onclick={on_increment}>{ "+1" }</button>
            <p>{ *counter }</p>
            <button onclick={on_decrement}>{ "-1" }</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
