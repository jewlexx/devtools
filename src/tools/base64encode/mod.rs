use std::ops::Deref;

use serde::Serialize;
use stylist::{
    yew::{styled_component, Global},
    StyleSource,
};
use yew::prelude::*;
use yewprint::{Button, Card, Elevation, Icon, IconName, InputGroup, Text};

use crate::{
    ffi::{clip_write, invoke},
    jsv,
};

#[derive(Debug, Clone, Serialize)]
struct Base64ParseArgs {
    pub input: String,
    pub encode: bool,
}

#[styled_component(Encode)]
pub fn encode() -> Html {
    let class_name: StyleSource = include_str!("mod.css").into();

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

    {
        let input = input.clone();
        let output = output.clone();

        use_effect_with_deps(
            |input| {
                let input = input[0].deref().to_owned();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = invoke(
                        "base64_parse",
                        jsv!(Base64ParseArgs {
                            input,
                            encode: true,
                        })
                        .unwrap(),
                    )
                    .await;

                    output.set(response.as_string().unwrap());
                });

                || {}
            },
            [input],
        );
    }

    html! {
        <>
            <Global css={class_name}/>
            <div class="container">
                <InputGroup
                    class={"input"}
                    placeholder={"Enter the text to encode in base64"}
                    value={input.deref().to_owned()}
                    oninput={input_changed}
                ></InputGroup>
                <Card class="output" elevation={Elevation::Level2}>
                    <Text>
                        { "Encoded Output: " } <code>{ if output.deref().is_empty() {
                            "Enter the text to encode in base64"
                        } else {
                            &*output
                        } }</code>
                    </Text>
                </Card>
                <Button class="copy-button" onclick={copy_output}><Icon icon={IconName::Clipboard}/></Button>
            </div>
        </>
    }
}