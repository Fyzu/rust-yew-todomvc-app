use std::sync::atomic::{AtomicUsize, Ordering};

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::todos::Todo;
use crate::{components::todo_list::TodoList, todos::Filter};
use yew::{function_component, html, use_state, Callback};

fn filter_todos(todos: &Vec<Todo>, filter: &Filter) -> Vec<Todo> {
    match filter {
        Filter::All => todos.clone(),
        Filter::Active => todos
            .iter()
            .filter(|todo| !todo.is_completed)
            .cloned()
            .collect(),
        Filter::Completed => todos
            .iter()
            .filter(|todo| todo.is_completed)
            .cloned()
            .collect(),
    }
}

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(1);

#[function_component(Application)]
pub fn application() -> Html {
    let active_filter = use_state(|| Filter::All);
    let todos = use_state(Vec::new);

    let on_add_todo = {
        let todos = todos.clone();
        Callback::from(move |text: String| {
            let mut values = (*todos).clone();
            values.push(Todo {
                id: GLOBAL_ID.fetch_add(1, Ordering::Relaxed),
                text: text.clone(),
                is_completed: false,
            });
            todos.set(values);
        })
    };

    let todos_count = (*todos).len();
    let completed_todos_count = (*todos).iter().filter(|todo| todo.is_completed).count();

    let on_select_filter = {
        let filter = active_filter.clone();
        Callback::from(move |new_filter| filter.set(new_filter))
    };

    let on_clear_completed = Callback::from(|_| {});

    let show_todos = filter_todos(&*todos, &*active_filter);

    html! {
        <div class="todoapp">
            <Header {on_add_todo} />

        <section class="main">
            <TodoList todos={show_todos} />
            if todos_count > 0 {
                <Footer {todos_count} {completed_todos_count} {on_select_filter} {on_clear_completed} active_filter={(*active_filter).clone()} />
            }
            </section>
        </div>
    }
}
