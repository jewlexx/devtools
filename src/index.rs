use strum::IntoEnumIterator;
use yew::prelude::*;
use yewprint::{Card, Elevation};

use crate::routing::AppRoute;

#[derive(Clone, PartialEq, Properties)]
struct RouteProps {
    route: AppRoute,
}

#[function_component(RouteDisplay)]
fn route_display(props: &RouteProps) -> Html {
    html! {
        <Card elevation={Elevation::Level2}>
            <h1>{&props.route.title}</h1>
            <p>{&props.route.description}</p>
        </Card>
    }
}

#[function_component(Index)]
pub fn index() -> Html {
    let routes = crate::routing::Route::iter()
        .map(|x| -> AppRoute { x.into() })
        .collect::<Vec<_>>();

    html! {
        <main class="container">
            <h1>{ "The developer's best friend" }</h1>
        </main>
    }
}
