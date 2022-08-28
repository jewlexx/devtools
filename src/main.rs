mod app;
mod routing;
mod index;

use app::App;

fn main() {
    yew::start_app::<App>();
}
