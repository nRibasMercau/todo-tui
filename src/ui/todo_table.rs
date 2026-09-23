use crate::app::{ActivePanel, App};
use crate::models::todo::Status;
use crate::ui::styles::{ACTIVE, INACTIVE};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Row, Table};

pub const DONE_ICON: &str = "✓";
pub const TODO_IN_PROGRESS_ICON: &str = "□";

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let active = app.active_panel == ActivePanel::Todos;
    let style = if active { ACTIVE } else { INACTIVE };

    let header = Row::new([
        "",
        "Project",
        "Task",
        "Status",
        "Created at",
        "Due date",
        "Completed at",
    ])
    .style(Style::new().bold())
    .bottom_margin(1);

    let rows = app.todo_table.items.iter().map(|todo| {
        Row::new([
            Span::raw(status_icon(todo.status)),
            Span::raw(todo.project.as_deref().unwrap_or("")),
            Span::styled(todo.todo.to_string(), todo_style(todo.status)),
            Span::styled(todo.status.to_string(), status_style(todo.status)),
            Span::raw(todo.created_at.format("%Y-%m-%d").to_string()),
            Span::raw(
                todo.due_date
                    .map(|date| date.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "".to_string()),
            ),
            Span::raw(
                todo.completed_at
                    .map(|date| date.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "".to_string()),
            ),
        ])
    });

    let widths = [
        Constraint::Percentage(1),
        Constraint::Percentage(10),
        Constraint::Percentage(44),
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
    ];
    let table = Table::new(rows, widths)
        .block(
            Block::default()
                .padding(Padding::vertical(1))
                .title(Span::styled("TODO", style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(style),
        )
        .header(header)
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(
            Style::new()
                .on_black()
                .bold()
                .add_modifier(Modifier::ITALIC),
        )
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(table, area, &mut app.todo_table.state);
}

fn status_style(status: Status) -> Style {
    Style::default().fg(status.color())
}

fn status_icon(status: Status) -> &'static str {
    match status {
        Status::ToDo => TODO_IN_PROGRESS_ICON,
        Status::InProgress => TODO_IN_PROGRESS_ICON,
        Status::Done => DONE_ICON,
    }
}

fn todo_style(status: Status) -> Style {
    match status {
        Status::Done => Style::default().add_modifier(Modifier::CROSSED_OUT),
        _ => Style::default(),
    }
}
