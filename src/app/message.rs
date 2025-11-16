use iced::widget::text_editor;
use iced::Event;

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(text_editor::Action),
    TabPressed,
    EnterPressed,
    CharacterTyped(char),
    BackspacePressed,
    DeletePressed,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    EventOccurred(Event),
    Tick,
}
