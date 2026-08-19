use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("q quit   j/k move    Spacebar change status     a add     o edit")
        .block(
            Block::default()
                .title("Help")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
