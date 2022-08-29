use std::ops::Deref;
use yew::prelude::*;
use yew_octicons::Icon;
use yew_octicons::IconKind;
use yewprint::{Button, InputGroup, Text};

use crate::ffi::clip_write;

#[function_component(Encode)]
pub fn encode() -> Html {
    let input = use_state(|| "".to_string());
    let output = use_state(|| "".to_string());

    let input_changed = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            input.set(value);
        })
    };

    let copy_output = {
        let output = output.clone();

        Callback::from(move |_| {
            clip_write(&*output);
        })
    };

    html! {
        <div class="container">
            <InputGroup
                placeholder={"Enter the text to encode in base64"}
                value={input.deref().to_owned()}
                oninput={input_changed}
            ></InputGroup>
            <Text class="base64-output">{ &*output }</Text>
            <Button class="copy-button" onclick={copy_output}><span>{ Icon::new(IconKind::Copy) }</span></Button>
        </div>
    }
}
