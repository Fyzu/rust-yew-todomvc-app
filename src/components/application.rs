use crate::components::content::Content;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::todos::Todo;
use yew::{function_component, html, use_state, Callback};

#[function_component(Application)]
pub fn application() -> Html {
    let todos = use_state(Vec::new);

    let on_add_todo = {
        let todos = todos.clone();
        Callback::from(move |text: String| {
            let todos = todos.clone();

            let mut values = (*todos).clone();
            values.push(Todo {
                text: text.clone(),
                is_completed: false,
            });
            todos.set(values);
        })
    };

    html! {
        <div class="todoapp">
            <Header {on_add_todo} />
            <Content todos={(*todos).clone()} />
            <Footer todos_count={(*todos).len()} />
        </div>
    }
}
