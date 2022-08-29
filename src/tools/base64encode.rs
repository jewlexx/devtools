use std::ops::Deref;
use yew::prelude::*;
use yew_octicons::Icon;
use yew_octicons::IconKind;
use yewprint::InputGroup;

use crate::ffi::clip_write;

#[function_component(Encode)]
pub fn encode() -> Html {
    let input = use_state(|| "".to_string());
    let output = use_state(|| "".to_string());
    let input_ref = use_ref(NodeRef::default);

    let input_changed = {
        let input = input.clone();
        let input_ref = input_ref.clone();
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

    {
        let input_ref = input.clone();
        let output = output.clone();

        use_effect_with_deps(
            move |_| {
                let input_value = &*input_ref;

                let output_value = base64::encode(input_value);

                output.set(output_value);

                || {}
            },
            [input.clone()],
        );
    }

    html! {
        <div class="container">
            <InputGroup
                placeholder={"Enter the text to encode in base64"}
                value={input.deref().to_owned()}
                oninput={input_changed}
            ></InputGroup>
            // <input type="text" label="Input" ref={&*input_ref} oninput={input_changed} />
            <p class="base64-output">{ &*output } <button class="copy-button" onclick={copy_output}><span>{ Icon::new(IconKind::Copy) }</span></button></p>
        </div>
    }
}
