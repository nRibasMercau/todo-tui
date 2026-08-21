use crate::app::{Focus, StringField, TodoPopup};
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

impl Widget for StringFieldWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [label_area, value_area] = area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
        ]));

        Line::from(self.string_field.label)
            .bold()
            .render(label_area, buf);

        let border_style = if self.is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let value_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(border_style)
            .padding(Padding::horizontal(1));

        let value_inner = value_block.inner(value_area);

        value_block.render(value_area, buf);
        Paragraph::new(self.string_field.value.as_str())
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

    let [todo_area, info_area, tag_area] = inner_area.layout(&Layout::vertical([
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
    ]));

    let todo_widget = StringFieldWidget {
        string_field: &todo_popup.todo,
        is_focused: todo_popup.focus == Focus::Todo,
    };
    let info_widget = StringFieldWidget {
        string_field: &todo_popup.info,
        is_focused: todo_popup.focus == Focus::Info,
    };
    let tag_widget = StringFieldWidget {
        string_field: &todo_popup.tag,
        is_focused: todo_popup.focus == Focus::Tag,
    };

    frame.render_widget(todo_widget, todo_area);
    frame.render_widget(info_widget, info_area);
    frame.render_widget(tag_widget, tag_area);
}
