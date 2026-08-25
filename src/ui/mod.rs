pub mod calendar;
mod footer;
mod todo_list;
mod todo_popup;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    let list_area = layout[0];
    let footer_area = layout[1];
    todo_list::render(app, frame, list_area);

    if let Some(popup) = &app.popup {
        todo_popup::render(popup, frame);
        footer::render(
            frame,
            footer_area,
            "Esc cancel     Tab move     Left/Right Toggle status      Enter save".to_string(),
        )
    } else {
        footer::render(
            frame,
            footer_area,
            "q/Esc quit   j/k move    Spacebar change status     a add     Enter edit".to_string(),
        );
    }
}
