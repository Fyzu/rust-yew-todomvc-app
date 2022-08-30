use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct CompleteButtonProps {
    pub value: bool,
    pub on_click: Callback<()>,
}

#[function_component(CompleteButton)]
pub fn complete_button(props: &CompleteButtonProps) -> Html {
    html! {
        <span>
          <input
            class="toggle-all"
            type="checkbox"
            checked={props.value}
          />
          <label onclick={props.on_click.reform(|_| ())} />
        </span>
    }
}
