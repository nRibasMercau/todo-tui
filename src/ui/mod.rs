mod footer;
mod todo_list;

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
    footer::render(frame, footer_area);
}
