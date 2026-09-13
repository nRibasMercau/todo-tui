use crate::models::project::{NewProject, Project};
use crate::ui::fields::StringField;
use ratatui::{
    Frame,
    buffer::Buffer,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget},
};

#[derive(Debug)]
pub struct ProjectPopup {
    pub id: Option<i64>,
    pub name: StringField,
    pub focus: Focus,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Name,
}

struct StringFieldWidget<'a> {
    string_field: &'a StringField,
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

impl ProjectPopup {
    pub fn new() -> Self {
        Self {
            id: None,
            name: StringField::blank("Name"),
            focus: Focus::Name,
        }
    }

    pub fn from_project(project: &Project) -> Self {
        Self {
            id: Some(project.id),
            name: StringField::new("Name", project.name.clone()),
            focus: Focus::Name,
        }
    }

    pub fn into_new_project(self) -> NewProject {
        NewProject {
            name: self.name.stringfield_to_string(),
            archived: false,
        }
    }

    pub fn render(project_popup: &ProjectPopup, frame: &mut Frame) {
        let area = frame.area();
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(10));

        frame.render_widget(Clear, centered_area);

        let block = Block::default()
            .title(if project_popup.id.is_some() {
                "Edit project"
            } else {
                "New project"
            })
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));

        let inner_area = block.inner(centered_area);

        frame.render_widget(block, centered_area);

        let name_widget = StringFieldWidget {
            string_field: &project_popup.name,
            is_focused: project_popup.focus == Focus::Name,
        };
        frame.render_widget(name_widget, inner_area);
    }
}
