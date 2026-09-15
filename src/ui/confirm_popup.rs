use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
};

#[derive(Debug)]
pub struct ConfirmPopup {
    pub title: String,
    pub message: String,
    pub selected: ConfirmChoice,
}

#[derive(Debug)]
pub enum ConfirmChoice {
    Yes,
    No,
}

impl ConfirmPopup {
    pub fn new(title: String, message: String) -> Self {
        Self {
            title,
            message,
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

    pub fn focus_next(&mut self) {
        match &self.selected {
            ConfirmChoice::Yes => self.selected = ConfirmChoice::No,
            ConfirmChoice::No => self.selected = ConfirmChoice::Yes,
        }
    }

    pub fn focus_previous(&mut self) {
        match &self.selected {
            ConfirmChoice::Yes => self.selected = ConfirmChoice::No,
            ConfirmChoice::No => self.selected = ConfirmChoice::Yes,
        }
    }
}
