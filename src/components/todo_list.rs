use yew::{function_component, html, Properties};

use crate::todos::Todo;
use crate::components::todo_item::TodoItem;

#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html! {
        <ul class="todo-list">
            {for props.todos.iter().map(|todo| {
                html! {
                    <TodoItem key={todo.id} todo={todo.clone()} />
                }
            })}
        </ul>
    }
}