use strum::IntoEnumIterator;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let routes = crate::routing::Route::iter().collect::<Vec<_>>();

    html! {
        <main class="container">
            <h1>{ "The developer's best friend" }</h1>
        </main>
    }
}
