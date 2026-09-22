use crate::app::project_list::ProjectListItem;
use crate::app::{ActivePanel, App};
use crate::ui::styles::{ACTIVE, INACTIVE};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding};

const HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let active = app.active_panel == ActivePanel::Projects;
    let style = if active { ACTIVE } else { INACTIVE };

    let projects = app.projects.items.iter().map(|project| match project {
        ProjectListItem::All => ListItem::new(Line::from("All")),
        ProjectListItem::Project(project) => ListItem::new(Line::from(format!("{}", project.name))),
    });

    let list = List::new(projects)
        .block(
            Block::default()
                .padding(Padding::vertical(1))
                .title(Span::styled("PROJECTS", style))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(style),
        )
        .style(Color::White)
        .highlight_style(HIGHLIGHT_STYLE)
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut app.projects.state);
}
