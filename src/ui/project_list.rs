use crate::app::ActivePanel;
use crate::app::App;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding};

const HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);
const LIST_HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Yellow);

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let projects = app
        .projects
        .items
        .iter()
        .map(|project| ListItem::new(Line::from(format!("{}", project.name))));

    let list = List::new(projects)
        .block(
            Block::default()
                .padding(Padding::vertical(1))
                .title(Span::styled("PROJECTS", title_style(&app.active_panel)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(
                    if &app.active_panel == &ActivePanel::Projects {
                        Color::Yellow
                    } else {
                        Color::White
                    },
                )),
        )
        .style(Color::White)
        .highlight_style(HIGHLIGHT_STYLE)
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut app.projects.state);
}

fn title_style(active_panel: &ActivePanel) -> Style {
    match active_panel {
        ActivePanel::Projects => LIST_HIGHLIGHT_STYLE,
        ActivePanel::Todos => Style::new(),
    }
}
