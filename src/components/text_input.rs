use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Properties};

#[derive(PartialEq, Properties)]
pub struct TextInputProps {
    pub value: String,
    pub on_input: Callback<String>,
    pub on_change: Callback<()>,
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

    let onchange = {
        let on_change = props.on_change.clone();

        Callback::from(move |_| on_change.emit(()))
    };

    html! {
        <input class="edit" type="text" placeholder={props.placeholder.clone()} value={props.value.clone()} {oninput} {onchange} />
    }
}
