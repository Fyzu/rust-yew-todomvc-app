#[derive(PartialEq, Clone)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub is_completed: bool,
}

#[derive(PartialEq, Clone)]
pub enum Filter {
    All,
    Active,
    Completed,
}
