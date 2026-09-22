use super::calendar;
use crate::models::todo::{NewTodo, Todo, TodoFormData};
use crate::models::todo::{Status, TodoId};
use crate::ui::fields::StringField;
use chrono::{Local, NaiveDate};
use ratatui::{
    Frame,
    buffer::Buffer,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget},
};
use ratatui_textarea::{CursorMove, TextArea, WrapMode};

struct StringFieldWidget<'a> {
    string_field: &'a StringField,
    is_focused: bool,
}

struct StatusFieldWidget {
    status: Status,
    is_focused: bool,
}

struct DueDateFieldWidget {
    due_date: Option<NaiveDate>,
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
            .style(Style::default().fg(self.status.color()))
            .render(value_inner, buf);
    }
}

impl Widget for DueDateFieldWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [label_area, value_area] = area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
        ]));

        Line::from("Due date").bold().render(label_area, buf);

        let due_date = match self.due_date {
            Some(date) => date.to_string(),
            None => "".to_string(),
        };

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
        Paragraph::new(due_date.to_string())
            .alignment(Alignment::Left)
            .render(value_inner, buf);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Todo,
    Info,
    Status,
    DueDate,
    Project,
}

#[derive(Debug)]
pub enum TodoPopupMode {
    Create,
    Edit(TodoId),
}

#[derive(Debug)]
pub struct TodoPopup {
    pub mode: TodoPopupMode,
    pub id: Option<i64>,
    pub todo: StringField,
    pub info: TextArea<'static>,
    pub status: Status,
    pub project: StringField,
    pub due_date: Option<NaiveDate>,
    pub focus: Focus,
    // TODO: calendar_date should be an Option
    pub calendar_date: NaiveDate,
}

impl<'a> TodoPopup {
    pub fn new() -> Self {
        Self {
            mode: TodoPopupMode::Create,
            id: None,
            todo: StringField::blank("To do"),
            info: TextArea::default(),
            project: StringField::blank("Project"),
            status: Status::ToDo,
            due_date: None,
            focus: Focus::Todo,
            calendar_date: Local::now().date_naive(),
        }
    }

    pub fn from_todo(todo: &Todo) -> Self {
        /*
         * TodoItem (borrowed)
         *   │
         *   ├── String ──────→ clone ──→ StringField
         *   ├── String ──────→ clone ──→ StringField
         *   ├── String ──────→ clone ──→ StringField
         *   ├── Status ───────→ copy ───→ Status
         *   └── NaiveDate ────→ copy ───→ NaiveDate
         */
        let mut info = TextArea::from(todo.info.lines().map(String::from).collect::<Vec<String>>());
        info.move_cursor(CursorMove::Bottom);
        info.move_cursor(CursorMove::End);

        Self {
            mode: TodoPopupMode::Edit(todo.id),
            id: Some(todo.id),
            todo: StringField::new("To do", todo.todo.clone()),
            info: info,
            project: StringField::new("Project", todo.project.clone().unwrap_or_default()),
            status: todo.status,
            due_date: todo.due_date,
            focus: Focus::Todo,
            calendar_date: match todo.due_date {
                Some(date) => date,
                None => Local::now().date_naive(),
            },
        }
    }

    pub fn into_new_todo(&self) -> NewTodo {
        NewTodo {
            todo: self.todo.stringfield_to_string(),
            info: self.info.lines().join("\n"),
            status: self.status,
            project: (!self.project.value.is_empty()).then(|| self.project.stringfield_to_string()),
            due_date: self.due_date,
        }
    }

    pub fn into_form_data(&self) -> TodoFormData {
        TodoFormData {
            todo: self.todo.stringfield_to_string(),
            info: self.info.lines().join("\n"),
            status: self.status,
            project: (!self.project.value.is_empty()).then(|| self.project.stringfield_to_string()),
            due_date: self.due_date,
        }
    }

    pub fn focus_next(&mut self) {
        match &self.focus {
            Focus::Todo => self.focus = Focus::Info,
            Focus::Info => self.focus = Focus::Status,
            Focus::Status => self.focus = Focus::DueDate,
            Focus::DueDate => self.focus = Focus::Project,
            Focus::Project => self.focus = Focus::Todo,
        }
    }

    pub fn focus_previous(&mut self) {
        match &self.focus {
            Focus::Todo => self.focus = Focus::Project,
            Focus::Info => self.focus = Focus::Todo,
            Focus::Status => self.focus = Focus::Info,
            Focus::DueDate => self.focus = Focus::Status,
            Focus::Project => self.focus = Focus::DueDate,
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

        let [todo_area, info_area, status_due_date_area, proyect_area] =
            inner_area.layout(&Layout::vertical([
                Constraint::Length(4),
                Constraint::Min(4),
                Constraint::Length(4),
                Constraint::Length(4),
            ]));

        let [status_area, due_date_area] = status_due_date_area.layout(&Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]));

        // Form
        // Todo
        let todo_widget = StringFieldWidget {
            string_field: &todo_popup.todo,
            is_focused: todo_popup.focus == Focus::Todo,
        };

        // Info
        let [info_label_area, info_value_area] = info_area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
        ]));
        let info_style: Style = if todo_popup.focus == Focus::Info {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let mut info_widget = todo_popup.info.clone();
        info_widget.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(info_style),
        );
        info_widget.set_wrap_mode(WrapMode::WordOrGlyph);
        // default line style
        info_widget.set_cursor_line_style(Style::default());
        // if focus is not Info, hide cursor in info TextArea
        if todo_popup.focus != Focus::Info {
            info_widget.set_cursor_style(Style::default());
        }

        // Status
        let status_widget = StatusFieldWidget {
            status: todo_popup.status,
            is_focused: todo_popup.focus == Focus::Status,
        };

        // Due date
        let due_date_widget = DueDateFieldWidget {
            due_date: todo_popup.due_date,
            is_focused: todo_popup.focus == Focus::DueDate,
        };

        // Project
        let proyect_widget = StringFieldWidget {
            string_field: &todo_popup.project,
            is_focused: todo_popup.focus == Focus::Project,
        };

        frame.render_widget(todo_widget, todo_area);
        frame.render_widget(
            Paragraph::new("Description").style(Style::new().add_modifier(Modifier::BOLD)),
            info_label_area,
        );
        frame.render_widget(&info_widget, info_value_area);
        frame.render_widget(status_widget, status_area);
        frame.render_widget(due_date_widget, due_date_area);
        frame.render_widget(proyect_widget, proyect_area);

        // Cursor position based on focus
        // In case of focus on Status or DueDate, the cursor is hidden
        // In case of Info, TextArea sets its own cursor
        let cursor_position = match &todo_popup.focus {
            Focus::Status | Focus::DueDate | Focus::Info => None,
            Focus::Todo => Some(todo_popup.todo.cursor_position(todo_area)),
            Focus::Project => Some(todo_popup.project.cursor_position(proyect_area)),
        };

        if let Some(position) = cursor_position {
            frame.set_cursor_position(position);
        }

        if todo_popup.focus == Focus::DueDate {
            let calendar_width = 60;
            let calendar_area = Rect {
                //x: due_date_area.x + (due_date_area.x - calendar_width) / 2,
                x: due_date_area.x,
                y: due_date_area.y + due_date_area.height,
                width: calendar_width,
                height: 10,
            };
            calendar::render(frame, calendar_area, todo_popup.calendar_date);
        }
    }
}
