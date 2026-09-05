use ratatui::{
    Frame,
    buffer::Buffer,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget},
};

#[derive(Debug)]
pub struct ConfirmPopup {
    pub message: String,
    pub selected: ConfirmChoice,
}

#[derive(Debug)]
pub enum ConfirmChoice {
    Yes,
    No,
}

impl ConfirmPopup {
    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(60));

        frame.render_widget(Clear, centered_area);

        let block = Block::default()
            .title("Todo")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));

        let inner_area = block.inner(centered_area);

        frame.render_widget(block, centered_area);

        let [message_area, options_area] = inner_area.layout(&Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(1),
        ]));

        let [yes_area, no_area] = options_area.layout(&Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]));
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
