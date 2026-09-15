pub mod calendar;
pub mod confirm_popup;
pub mod fields;
pub mod footer;
pub mod project_list;
pub mod project_popup;
pub mod todo_list;
pub mod todo_popup;
use crate::app::{ActivePanel, App, Dialog};
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

    if let Some(dialog) = &app.dialog {
        match dialog {
            Dialog::Todo(popup) => {
                todo_popup::TodoPopup::render(popup, frame);
                footer::render(
                    frame,
                    footer_area,
                    "Esc cancel     Tab move     Left/Right Toggle status      Enter save"
                        .to_string(),
                )
            }
            Dialog::Project(popup) => {
                project_popup::ProjectPopup::render(popup, frame);
                footer::render(frame, footer_area, "Esc cancel     Enter save".to_string())
            }
            Dialog::Confirm(popup) => {
                confirm_popup::ConfirmPopup::render(popup, frame);
                footer::render(
                    frame,
                    footer_area,
                    "Y confirm      N/Esc cancel".to_string(),
                )
            }
            _ => {}
        }
    } else {
        match app.active_panel {
            ActivePanel::Todos => {
                footer::render(
                    frame,
                    footer_area,
                    "q/Esc quit   j/k move    Spacebar change status     a add     d delete    Enter edit"
                        .to_string(),
                );
            }
            ActivePanel::Projects => {
                footer::render(
                    frame,
                    footer_area,
                    "q/Esc quit   j/k move    a add     d delete   d delete   Enter edit"
                        .to_string(),
                );
            }
        }
    }
}
