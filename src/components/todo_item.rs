use yew::{function_component, html, Properties, use_state};

use crate::todos::Todo;

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: Todo,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let is_editing = use_state(|| false);

    let view = {
        html! {
            {props.todo.text.clone()}
        }
    };

    let class = {
        if *is_editing {
            "editing"
        } else if props.todo.is_completed {
            "completed"
        } else {
            ""
        }
    };

    html! {
        <li {class}>
            {view}
        </li>
    }
}