use yew::prelude::*;

use crate::ffi::log;

#[function_component(Encode)]
pub fn encode() -> Html {
    let input = use_state(|| "".to_string());

    let set_input = {
        let input = input.clone();

        move |v: Event| {
            input.set(v.as_string().unwrap_or_default());
        }
    };

    html! {
        <div class="container">
            <input label="Input" onchange={Callback::from(set_input)} />
            <p>{ &*input }</p>
        </div>
    }
}
