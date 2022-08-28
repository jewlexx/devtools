use strum::EnumIter;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq, EnumIter)]
pub enum Route {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}
