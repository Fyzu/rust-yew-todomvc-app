use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct FooterProps {
    pub todos_count: usize,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    html! {
        <footer class="footer">
            <span class="todo-count">{props.todos_count}</span>
            <ul class="filters">
            </ul>
            <button class="clear-completed">
                {"Clear completed"}
            </button>
        </footer>
    }
}
