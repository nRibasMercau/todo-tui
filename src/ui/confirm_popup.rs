use crate::models::{project::ProjectId, todo::TodoId};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
};

#[derive(Debug)]
pub struct ConfirmPopup {
    pub title: String,
    pub message: String,
    pub action: ConfirmAction,
    pub selected: ConfirmChoice,
}

#[derive(Debug)]
pub enum ConfirmChoice {
    Yes,
    No,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeleteProject(ProjectId),
    DeleteTodo(TodoId),
}

impl ConfirmPopup {
    pub fn new(title: String, message: String, action: ConfirmAction) -> Self {
        Self {
            title,
            message,
            action,
            selected: ConfirmChoice::No,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let centered_area = area.centered(Constraint::Percentage(40), Constraint::Percentage(20));

        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));

        let text = format!("{}\n\nPress 'y' for Yes, 'n' for No", self.message);
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(Clear, centered_area);
        frame.render_widget(paragraph, centered_area);
    }
}
