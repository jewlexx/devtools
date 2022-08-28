use yew::prelude::*;

#[function_component(Encode)]
pub fn encode() -> Html {
    let input = use_state(|| "".to_string());
    let output = use_state(|| "".to_string());
    let input_ref = use_ref(NodeRef::default);

    let input_changed = {
        let input = input.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            input.set(
                input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
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
            [input],
        );
    }

    html! {
        <div class="container">
            <input type="text" label="Input" ref={&*input_ref} oninput={input_changed} />
            <p>{ &*output } <button>{ "📋" }</button></p>
        </div>
    }
}
