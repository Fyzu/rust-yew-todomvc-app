use yew::{function_component, html, Callback, Properties};

use crate::components::todo_item::TodoItem;
use crate::todos::Todo;

#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
    pub on_update_todo_is_completed: Callback<(usize, bool)>,
    pub on_update_todo_text: Callback<(usize, String)>,
    pub on_remove_todo: Callback<usize>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html! {
        <ul class="todo-list">
            {for props.todos.iter().map(|todo| {
                html! {
                    <TodoItem
                        key={todo.id}
                        todo={todo.clone()}
                        on_update_todo_is_completed={&props.on_update_todo_is_completed}
                        on_update_todo_text={&props.on_update_todo_text}
                        on_remove_todo={&props.on_remove_todo}
                    />
                }
            })}
        </ul>
    }
}
