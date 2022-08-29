use yew::{function_component, html, Properties};

use crate::todos::Todo;

#[derive(PartialEq, Properties)]
pub struct ContentProps {
    pub todos: Vec<Todo>,
}

#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
    html! {
        <section class="main">
            {for props.todos.iter().map(|todo| {
                html! {
                    <div>{todo.text.clone()}</div>
                }
            })}
        </section>
    }
}