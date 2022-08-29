use strum::EnumIter;
use yew_router::prelude::*;

pub struct AppRoute<T: yew::FunctionProvider + 'static> {
    pub route: Route,
    pub title: &'static str,
    pub description: &'static str,
    pub component: yew::functional::FunctionComponent<T>,
}

#[derive(Clone, Routable, PartialEq, Eq, EnumIter)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/base64encode")]
    Base64Encode,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl<T: yew::FunctionProvider + 'static> From<Route> for AppRoute<T> {
    fn from(route: Route) -> Self {}
}
