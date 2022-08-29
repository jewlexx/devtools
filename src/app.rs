use crate::routing::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routing::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(|route: Route| {
                let app_route: AppRoute = route.into();

                app_route.component
            })} />
        </BrowserRouter>
    }
}
