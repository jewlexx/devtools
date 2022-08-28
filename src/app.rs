use crate::index::Index;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routing::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(|routes| {
                    match routes {
                        Route::Index => html! { <Index /> },
                        Route::NotFound => html! { <p>{"404"}</p> },
                    }
            })} />
        </BrowserRouter>
    }
}
