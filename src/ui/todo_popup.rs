use crate::app::{Focus, Status, StringField, TodoPopup};
use ratatui::{
    Frame,
    buffer::Buffer,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget},
};

struct StringFieldWidget<'a> {
    string_field: &'a StringField,
    is_focused: bool,
}

struct StatusFieldWidget {
    status: Status,
    is_focused: bool,
}

impl Widget for StringFieldWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [label_area, value_area] = area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
        ]));

        Line::from(self.string_field.label)
            .bold()
            .render(label_area, buf);

        let border_style: Style;

        // Yellow border when focused
        if self.is_focused {
            border_style = Style::default().fg(Color::Yellow);
        } else {
            border_style = Style::default();
        }

        let value_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .padding(Padding::horizontal(1));

        let value_inner = value_block.inner(value_area);

        value_block.render(value_area, buf);
        Paragraph::new(self.string_field.value.as_str())
            .alignment(Alignment::Left)
            .render(value_inner, buf);
    }
}

impl Widget for StatusFieldWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [label_area, value_area] = area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
        ]));

        Line::from("Status").bold().render(label_area, buf);

        // Yellow border when focused
        let border_style = if self.is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let value_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .padding(Padding::horizontal(1));

        let value_inner = value_block.inner(value_area);

        value_block.render(value_area, buf);
        Paragraph::new(self.status.to_string())
            .alignment(Alignment::Left)
            .render(value_inner, buf);
    }
}

pub fn render(todo_popup: &TodoPopup, frame: &mut Frame) {
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

    let [todo_area, info_area, status_area, tag_area] = inner_area.layout(&Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(4),
        Constraint::Length(4),
        Constraint::Length(4),
    ]));

    // Form
    let todo_widget = StringFieldWidget {
        string_field: &todo_popup.todo,
        is_focused: todo_popup.focus == Focus::Todo,
    };
    let info_widget = StringFieldWidget {
        string_field: &todo_popup.info,
        is_focused: todo_popup.focus == Focus::Info,
    };
    let status_widget = StatusFieldWidget {
        status: todo_popup.status,
        is_focused: todo_popup.focus == Focus::Status,
    };
    let tag_widget = StringFieldWidget {
        string_field: &todo_popup.tag,
        is_focused: todo_popup.focus == Focus::Tag,
    };
    frame.render_widget(todo_widget, todo_area);
    frame.render_widget(info_widget, info_area);
    frame.render_widget(status_widget, status_area);
    frame.render_widget(tag_widget, tag_area);

    // Cursor position based on focus
    // In case of focus on Status, the cursor is hidden
    let cursor_position = match &todo_popup.focus {
        Focus::Todo => Some(todo_popup.todo.cursor_position(todo_area)),
        Focus::Info => Some(todo_popup.info.cursor_position(info_area)),
        Focus::Tag => Some(todo_popup.tag.cursor_position(tag_area)),
        Focus::Status => None,
    };
    if let Some(position) = cursor_position {
        frame.set_cursor_position(position);
    }
}
