use crate::models::project::{Project, ProjectId};
use ratatui::widgets::ListState;

#[derive(Debug)]
pub enum ProjectListItem {
    All,
    Project(Project),
}

#[derive(Debug)]
pub struct ProjectList {
    pub items: Vec<ProjectListItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum ProjectListError {
    ProjectNotFound,
}

impl ProjectList {
    pub fn new(items: Vec<ProjectListItem>) -> Self {
        let mut state = ListState::default();

        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { items, state }
    }

    /// Selects next element in the list
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            self.state.select(None)
        } else {
            self.state.select_next();
        }
    }

    /// Selects previous element in the list
    pub fn select_previous(&mut self) {
        if self.items.is_empty() {
            self.state.select(None)
        } else {
            self.state.select_previous();
        }
    }
    pub fn replace_projects(&mut self, projects: Vec<Project>) {
        self.items = std::iter::once(ProjectListItem::All)
            .chain(projects.into_iter().map(ProjectListItem::Project))
            .collect();
    }

    pub fn selected_project_id(&self) -> Option<ProjectId> {
        match self.items.get(self.state.selected().unwrap_or(0)) {
            Some(ProjectListItem::All) => None,
            Some(ProjectListItem::Project(project)) => Some(project.id),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::project::Project;
    use chrono::Local;
    use color_eyre::Result;

    #[test]
    fn project_list_select_next() -> Result<()> {
        let mut projects = ProjectList::new(vec![
            ProjectListItem::Project(Project {
                id: 1,
                name: String::from("Project 1"),
                archived: false,
                created_at: Local::now().date_naive(),
            }),
            ProjectListItem::Project(Project {
                id: 2,
                name: String::from("Project 2"),
                archived: false,
                created_at: Local::now().date_naive(),
            }),
        ]);

        projects.state.select(Some(0));

        projects.select_next();
        assert_eq!(projects.state.selected(), Some(1));

        projects.select_next();
        assert_eq!(projects.state.selected(), Some(2));

        projects.select_next();
        assert_eq!(projects.state.selected(), Some(3));

        Ok(())
    }

    #[test]
    fn project_list_select_previous() -> Result<()> {
        let mut projects = ProjectList::new(vec![
            ProjectListItem::Project(Project {
                id: 1,
                name: String::from("Project 1"),
                archived: false,
                created_at: Local::now().date_naive(),
            }),
            ProjectListItem::Project(Project {
                id: 2,
                name: String::from("Project 2"),
                archived: false,
                created_at: Local::now().date_naive(),
            }),
        ]);

        projects.state.select(Some(2));

        projects.select_previous();
        assert_eq!(projects.state.selected(), Some(1));

        projects.select_previous();
        assert_eq!(projects.state.selected(), Some(0));

        projects.select_previous();
        assert_eq!(projects.state.selected(), Some(0));

        Ok(())
    }

    #[test]
    fn empty_list_select_next() -> Result<()> {
        let mut projects = ProjectList::new(vec![]);

        assert_eq!(projects.state, ListState::default());

        projects.select_next();
        assert_eq!(projects.state.selected(), None);

        projects.select_next();
        assert_eq!(projects.state.selected(), None);

        Ok(())
    }

    #[test]
    fn empty_list_select_previous() -> Result<()> {
        let mut projects = ProjectList::new(vec![]);

        assert_eq!(projects.state, ListState::default());

        projects.select_previous();
        assert_eq!(projects.state.selected(), None);

        projects.select_previous();
        assert_eq!(projects.state.selected(), None);

        Ok(())
    }
}
