mod app;
mod ffi;
mod index;
mod macros;
mod not_found;
mod routing;
mod tools;

use app::App;

fn main() {
    yew::start_app::<App>();
}
