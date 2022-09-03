use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent};
use yew::{function_component, html, Callback, Classes, NodeRef, Properties};

#[derive(PartialEq, Properties)]
pub struct TextInputProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    pub value: String,
    pub on_input: Callback<String>,
    #[prop_or_default]
    pub on_blur: Callback<()>,
    pub on_enter: Callback<()>,
    pub placeholder: String,
}

#[function_component(TextInput)]
pub fn textInput(props: &TextInputProps) -> Html {
    let oninput = {
        let on_input = props.on_input.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    on_input.emit(input.value());
                }
            }
        })
    };

    let on_keydown = {
        let on_enter = props.on_enter.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.which() == 13 {
                on_enter.emit(())
            }
        })
    };

    html! {
        <input
            ref={props.node_ref.clone()}
            class={props.class.clone()}
            type="text" placeholder={props.placeholder.clone()}
            value={props.value.clone()}
            {oninput}
            onblur={props.on_blur.reform(|_| ())}
            onkeydown={on_keydown}
        />
    }
}
