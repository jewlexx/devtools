use strum::IntoEnumIterator;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <main class="container">
            <h1>{ "The developer's best friend" }</h1>
            <a href="/base64encode">{ "Base64 Encode" }</a>
        </main>
    }
}
