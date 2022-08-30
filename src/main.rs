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

impl From<u8> for U4 {
    fn from(val: u8) -> Self {
        if val > 15 {
            panic!("U4 overflow");
        }

        let mut inner = [false; 4];
        inner.iter_mut().enumerate().for_each(|(i, b)| {
            *b = (val & (1 << i)) != 0;
        });
        inner.reverse();

        U4 { inner }
    }
}

impl From<U4> for u8 {
    fn from(val: U4) -> Self {
        let mut res = 0;
        for i in 0..4 {
            if val.inner[i] {
                res += 1 << i;
            }
        }
        res
    }
}

impl Add<u8> for U4 {
    type Output = U4;

    fn add(self, rhs: u8) -> Self::Output {
        let rhs = rhs;
        let decimal: u8 = self.into();

        let res = rhs + decimal;

        if res > 15 {
            panic!("U4 overflow");
        }

        Self::from(res)
    }
}

fn main() {
    yew::start_app::<App>();
}
