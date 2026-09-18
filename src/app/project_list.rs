use crate::models::project::Project;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct ProjectList {
    pub items: Vec<Project>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum ProjectListError {
    ProjectNotFound,
}

impl ProjectList {
    pub fn new(items: Vec<Project>) -> Self {
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

    pub fn add_project(&mut self, project: Project) {
        self.items.push(project);
    }

    pub fn replace_project(&mut self, project: Project) -> Result<(), ProjectListError> {
        match self.items.iter_mut().find(|i| i.id == project.id) {
            Some(item) => {
                *item = project;
                Ok(())
            }
            None => Err(ProjectListError::ProjectNotFound),
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
            Project {
                id: 1,
                name: String::from("Project 1"),
                archived: false,
                created_at: Local::now().date_naive(),
            },
            Project {
                id: 2,
                name: String::from("Project 2"),
                archived: false,
                created_at: Local::now().date_naive(),
            },
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
            Project {
                id: 1,
                name: String::from("Project 1"),
                archived: false,
                created_at: Local::now().date_naive(),
            },
            Project {
                id: 2,
                name: String::from("Project 2"),
                archived: false,
                created_at: Local::now().date_naive(),
            },
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
