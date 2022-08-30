use crate::{components::filter_button::FilterButton, todos::Filter};
use yew::{function_component, html, Callback, Properties};

#[derive(PartialEq, Properties)]
pub struct FooterProps {
    pub todos_count: usize,
    pub completed_todos_count: usize,
    pub active_filter: Filter,
    pub on_select_filter: Callback<Filter>,
    pub on_clear_completed: Callback<()>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let active_count_text = {
        let todos_count = props.todos_count - props.completed_todos_count;
        match todos_count {
            0 => "No items".to_owned(),
            1 => "1 item".to_owned(),
            _ => format!("{todos_count} items"),
        }
    };

    let on_click = &props.on_select_filter;

    let active_filter = props.active_filter.clone();

    let on_clear_completed = {
        let on_click = props.on_clear_completed.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    html! {
        <footer class="footer">
            <span class="todo-count">{format!("{active_count_text} left")}</span>
            <ul class="filters">
                <li>
                    <FilterButton value={Filter::All} is_active={active_filter == Filter::All} {on_click}>
                        {"All"}
                    </FilterButton>
                </li>
                <li>
                <FilterButton value={Filter::Active} is_active={active_filter == Filter::Active} {on_click}>
                    {"Active"}
                    </FilterButton>
                </li>
                <li>
                <FilterButton value={Filter::Completed} is_active={active_filter == Filter::Completed} {on_click}>
                    {"Completed"}
                    </FilterButton>
                </li>
            </ul>
            if props.completed_todos_count > 0 {
                <button class="clear-completed" onclick={on_clear_completed}>
                    {"Clear completed"}
                </button>
            }
        </footer>
    }
}
