use yew::prelude::*;
use yew_router::prelude::*;

pub struct AppRoute {
    pub route: Route,
    pub title: &'static str,
    pub description: &'static str,
    pub component: fn() -> Html,
}

#[derive(Clone, Copy, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/base64encode")]
    Base64Encode,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl From<Route> for AppRoute {
    fn from(route: Route) -> Self {
        match route {
            Route::Index => AppRoute {
                route,
                title: "Home",
                description: "Home page",
                component: || html! { <crate::index::Index/> },
            },
            Route::Base64Encode => AppRoute {
                route,
                title: "Base64 Encode",
                description: "Base64 Encode page",
                component: || html! { <crate::tools::base64encode::Encode/> },
            },
            Route::NotFound => AppRoute {
                route,
                title: "404",
                description: "404 page",
                component: || html! { <crate::not_found::NotFound/> },
            },
        }
    }
}
