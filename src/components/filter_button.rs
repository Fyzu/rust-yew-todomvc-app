use yew::{function_component, html, Callback, Children, Properties};

use crate::todos::Filter;

#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub is_active: bool,
    pub value: Filter,
    pub on_click: Callback<Filter>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(FilterButton)]
pub fn filter_button(props: &ListProps) -> Html {
    let class = {
        if props.is_active {
            "selected"
        } else {
            ""
        }
    };

    let onclick = {
        let filter = props.value.clone();
        let on_click = props.on_click.clone();
        Callback::from(move |_| on_click.emit(filter.clone()))
    };

    html! {
        <a href="javascript:void(0)" type="button" role="switch" aria-checked={props.is_active.to_string()} {class} {onclick}>
            {for props.children.iter()}
        </a>
    }
}
