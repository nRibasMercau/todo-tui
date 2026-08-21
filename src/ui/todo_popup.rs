use crate::app::TodoPopup;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
};

pub fn render(todo_popup: &TodoPopup, frame: &mut Frame) {
    let area = frame.area();
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(60));

    frame.render_widget(Clear, centered_area);

    let block = Block::default()
        .title("Todo")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::uniform(1));

    let paragraph = Paragraph::new(vec![
        Line::from(format!("Task: {}", todo_popup.todo)),
        Line::from(format!("Description: {}", todo_popup.info)),
        Line::from(format!("Status: {}", todo_popup.status)),
        Line::from(format!("Project: {}", todo_popup.tag)),
    ])
    .block(block);

    frame.render_widget(paragraph, centered_area);
}
