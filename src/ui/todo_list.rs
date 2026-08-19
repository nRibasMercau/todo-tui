use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::app::{App, Status};

const HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);
const TODO_STYLE: Style = Style::new();
const INPROGRESS_STYLE: Style = Style::new().fg(Color::Yellow);
const DONE_STYLE: Style = Style::new().fg(Color::Green);
const DONE_ITEM_STYLE: Style = Style::new().add_modifier(Modifier::CROSSED_OUT);

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let items = app.todo_list.items.iter().map(|item| {
        ListItem::new(Line::from(vec![
            Span::raw(format!("{} ", status_icon(item.status))),
            Span::styled(format!("{} ", item.status), status_style(item.status)),
            Span::styled(format!("{}", item.todo), item_style(item.status)),
            Span::raw(format!(" {}", item.tag)),
        ]))
    });

    let list = List::new(items)
        .block(
            Block::default()
                .title("Todo list")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(HIGHLIGHT_STYLE)
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut app.todo_list.state);
}

fn status_icon(status: Status) -> &'static str {
    match status {
        Status::ToDo | Status::InProgress => "□",
        Status::Done => "✓",
    }
}

fn status_style(status: Status) -> Style {
    match status {
        Status::ToDo => TODO_STYLE,
        Status::InProgress => INPROGRESS_STYLE,
        Status::Done => DONE_STYLE,
    }
}

fn item_style(status: Status) -> Style {
    match status {
        Status::Done => DONE_ITEM_STYLE,
        _ => Style::default(),
    }
}
