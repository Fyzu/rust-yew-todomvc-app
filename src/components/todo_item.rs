use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};
use yew::{function_component, html, use_state, Callback, Properties};

use crate::components::text_input::TextInput;
use crate::todos::Todo;

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: Todo,
    pub on_update_todo_is_completed: Callback<(usize, bool)>,
    pub on_update_todo_text: Callback<(usize, String)>,
    pub on_remove_todo: Callback<usize>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let id = props.todo.id;
    let is_editing = use_state(|| false);
    let text = props.todo.text.clone();
    let editing_text = use_state(|| text.clone());

    let is_completed = props.todo.is_completed;

    let on_edit = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| is_editing.set(true))
    };

    let on_input_text = {
        let editing_text = editing_text.clone();
        Callback::from(move |value| editing_text.set(value))
    };

    let on_change_text = {
        let on_update_todo_text = props.on_update_todo_text.clone();
        let is_editing = is_editing.clone();
        let editing_text = editing_text.clone();

        Callback::from(move |_| {
            is_editing.set(false);
            on_update_todo_text.emit((id, (*editing_text).clone()))
        })
    };

    let on_completed_change = {
        let on_update_todo_is_completed = props.on_update_todo_is_completed.clone();
        Callback::from(move |event: Event| {
            if let Some(target) = event.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    let is_completed = input.checked();
                    on_update_todo_is_completed.emit((id, is_completed))
                }
            }
        })
    };

    let on_remove = {
        let on_remove_todo = props.on_remove_todo.clone();
        Callback::from(move |_| {
            on_remove_todo.emit(id)
        })
    };

    let view = {
        html! {
            if *is_editing {
                <TextInput class="edit" value={(*editing_text).clone()} placeholder="" on_input={on_input_text} on_change={on_change_text} />
        } else {
                <div className="view">
                    <input
                        class="toggle"
                        type="checkbox"
                        checked={is_completed}
                        onchange={on_completed_change}
                    />
                    <label ondblclick={on_edit}>{text}</label>
                    <button class="destroy" onclick={on_remove} />
                </div>
            }
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
