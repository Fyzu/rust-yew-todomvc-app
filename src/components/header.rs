use crate::components::text_input::TextInput;
use yew::{function_component, html, use_state_eq, Callback, Properties};

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub on_add_todo: Callback<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let todo_text = use_state_eq(|| "".to_owned());

    let handle_todo_text_change = {
        let todo_text = todo_text.clone();
        Callback::from(move |value: String| todo_text.set(value.clone()))
    };

    let handle_add_todo = {
        let todo_text = todo_text.clone();
        let on_add_todo = props.on_add_todo.clone();

        Callback::from(move |_| {
            let todo_text_value = (*todo_text).clone();
            on_add_todo.emit(todo_text_value);
            todo_text.set("".to_owned());
        })
    };

    html! {
        <header class="header">
            <h1>{"todos"}</h1>
            <TextInput value={(*todo_text).clone()} placeholder="What needs to be done?" on_input={handle_todo_text_change} on_change={handle_add_todo} />
        </header>
    }
}
