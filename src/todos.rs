use yew::Properties;

#[derive(PartialEq, Properties, Clone)]
pub struct Todo {
    pub text: String,
    pub is_completed: bool,
}
