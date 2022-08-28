mod app;
mod ffi;
mod index;
mod routing;

use app::App;

fn main() {
    yew::start_app::<App>();
}
