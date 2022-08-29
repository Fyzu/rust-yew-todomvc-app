mod components;
mod todos;

use components::application::Application;

fn main() {
    yew::start_app::<Application>();
}
