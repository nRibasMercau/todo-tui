pub mod calendar;
pub mod fields;
pub mod footer;
pub mod project_list;
pub mod project_popup;
pub mod todo_list;
pub mod todo_popup;
use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    let content_area = layout[0];
    let footer_area = layout[1];

    let [projects_area, todos_area] = content_area.layout(&Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]));

    todo_list::render(app, frame, todos_area);
    project_list::render(app, frame, projects_area);

    if let Some(popup) = &app.popup {
        todo_popup::TodoPopup::render(popup, frame);
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
