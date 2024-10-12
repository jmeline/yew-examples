use yew::prelude::*;

#[function_component]
fn App() -> Html {

    html! {
        <div>
            <h1>{"Hello, World!"}</h1>
            <p>{"This is a simple Yew app."}</p>
        </div>
    }

}

fn main() {
    yew::Renderer::<App>::new().render();
}
