use quill::{App};
use quill::ui::view;

fn main() -> iced::Result {
    iced::application("Quill", App::update, view)
        .subscription(App::subscription)
        .run()
}