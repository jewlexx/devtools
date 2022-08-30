mod app;
mod ffi;
mod index;
mod macros;
mod not_found;
mod routing;
mod tools;

use std::ops::Add;

use app::App;

struct U4 {
    inner: [bool; 4],
}

impl Add<u8> for U4 {
    type Output = U4;

    fn add(self, rhs: u8) -> Self::Output {
        if rhs > 15 {
            panic!("U4 overflow");
        }

        let mut inner = [false; 4];
        for i in 0..4 {
            inner[i] = rhs & (1 << i) != 0;
        }
        U4 { inner }
    }
}

fn main() {
    yew::start_app::<App>();
}
