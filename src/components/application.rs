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

    let on_clear_completed = {
        let todos = todos.clone();
        Callback::from(move |_| {
            let values = filter_todos(&*todos, &Filter::Active);

            todos.set(values);
        })
    };

    let on_update_todo_is_completed = {
        let todos = todos.clone();
        Callback::from(move |(id, is_completed)| {
            let mut values = (*todos).clone();
            if let Some((index, _)) = values.iter().enumerate().find(|(_, todo)| todo.id == id) {
                let mut new_todo = &mut values[index];
                new_todo.is_completed = is_completed;

                todos.set(values);
            }
        })
    };
    let on_update_todo_text = {
        let todos = todos.clone();
        Callback::from(move |(id, text)| {
            let mut values = (*todos).clone();
            if let Some((index, _)) = values.iter().enumerate().find(|(_, todo)| todo.id == id) {
                let mut new_todo = &mut values[index];
                new_todo.text = text;

                todos.set(values);
            }
        })
    };

    let on_remove_todo = {
        let todos = todos.clone();
        Callback::from(move |id| {
            let mut values = (*todos).clone();
            if let Some((index, _)) = values.iter().enumerate().find(|(_, todo)| todo.id == id) {
                values.remove(index);

                todos.set(values);
            }
        })
    };

    let show_todos = filter_todos(&*todos, &*active_filter);

    html! {
        <div class="todoapp">
            <Header {on_add_todo} />

        <section class="main">
            <TodoList todos={show_todos} {on_update_todo_text} {on_update_todo_is_completed} {on_remove_todo} />
            if todos_count > 0 {
                <Footer {todos_count} {completed_todos_count} {on_select_filter} {on_clear_completed} active_filter={(*active_filter).clone()} />
            }
            </section>
        </div>
    }
}
