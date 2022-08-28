use material_yew::text_inputs::MatTextField;
use yew::prelude::*;

use crate::ffi::log;

#[function_component(Encode)]
pub fn encode() -> Html {
    html! {
        <div class="container">
            <MatTextField outlined=true label="Input" oninput={Callback::from(|v: String| {log(&v);})} />
        </div>
    }
}
