use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="not-found">
            <h1>{ "404" }</h1>
            <p>{ "Page not found" }</p>
        </div>
    }
}
