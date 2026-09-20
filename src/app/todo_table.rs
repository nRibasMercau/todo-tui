use crate::models::todo::Todo;
use ratatui::widgets::TableState;

#[derive(Debug)]
pub struct TodoTable {
    pub items: Vec<Todo>,
    pub state: TableState,
}

impl TodoTable {
    pub fn new(items: Vec<Todo>) -> Self {
        let mut state = TableState::default();

        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { items, state }
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            self.state.select(None);
            return;
        }

        let next = match self.state.selected() {
            Some(i) if i + 1 < self.items.len() => i + 1,
            _ => 0,
        };

        self.state.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        if self.items.is_empty() {
            self.state.select(None);
            return;
        }

        let previous = match self.state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => self.items.len() - 1,
        };

        self.state.select(Some(previous));
    }

    pub fn toggle_status(&mut self, index: usize) {
        self.items[index].status = self.items[index].status.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::todo::{Status, Todo};
    use chrono::Local;
    use color_eyre::Result;

    #[test]
    fn todo_table_select_next() -> Result<()> {
        let mut todo_table = TodoTable::new(vec![
            Todo {
                id: 1,
                todo: String::from("Todo 1"),
                info: String::from("My todo 1"),
                status: Status::ToDo,
                project_id: None,
                project: None,
                due_date: None,
                created_at: Local::now().date_naive(),
                completed_at: None,
            },
            Todo {
                id: 2,
                todo: String::from("Todo 2"),
                info: String::from("My todo 2"),
                status: Status::ToDo,
                project_id: None,
                project: None,
                due_date: None,
                created_at: Local::now().date_naive(),
                completed_at: None,
            },
        ]);

        todo_table.state.select(Some(0));

        todo_table.select_next();
        assert_eq!(todo_table.state.selected(), Some(1));

        todo_table.select_next();
        assert_eq!(todo_table.state.selected(), Some(0));

        todo_table.select_next();
        assert_eq!(todo_table.state.selected(), Some(1));

        Ok(())
    }

    #[test]
    fn todo_table_select_previous() -> Result<()> {
        let mut todo_table = TodoTable::new(vec![
            Todo {
                id: 1,
                todo: String::from("Todo 1"),
                info: String::from("My todo 1"),
                status: Status::ToDo,
                project_id: None,
                project: None,
                due_date: None,
                created_at: Local::now().date_naive(),
                completed_at: None,
            },
            Todo {
                id: 2,
                todo: String::from("Todo 2"),
                info: String::from("My todo 2"),
                status: Status::ToDo,
                project_id: None,
                project: None,
                due_date: None,
                created_at: Local::now().date_naive(),
                completed_at: None,
            },
        ]);

        todo_table.state.select(Some(1));

        todo_table.select_previous();
        assert_eq!(todo_table.state.selected(), Some(0));

        todo_table.select_previous();
        assert_eq!(todo_table.state.selected(), Some(1));

        todo_table.select_previous();
        assert_eq!(todo_table.state.selected(), Some(0));

        Ok(())
    }

    #[test]
    fn empty_list_select_next() -> Result<()> {
        let mut todo_table = TodoTable::new(vec![]);

        assert_eq!(todo_table.state, TableState::default());

        todo_table.select_next();
        assert_eq!(todo_table.state.selected(), None);

        todo_table.select_next();
        assert_eq!(todo_table.state.selected(), None);

        Ok(())
    }

    #[test]
    fn empty_list_select_previous() -> Result<()> {
        let mut todo_table = TodoTable::new(vec![]);

        assert_eq!(todo_table.state, TableState::default());

        todo_table.select_previous();
        assert_eq!(todo_table.state.selected(), None);

        todo_table.select_previous();
        assert_eq!(todo_table.state.selected(), None);

        Ok(())
    }

    #[test]
    fn toggles_status() -> Result<()> {
        let mut todo_table = TodoTable::new(vec![
            Todo {
                id: 1,
                todo: String::from("Todo 1"),
                info: String::from("My todo 1"),
                status: Status::ToDo,
                project_id: None,
                project: None,
                due_date: None,
                created_at: Local::now().date_naive(),
                completed_at: None,
            },
            Todo {
                id: 2,
                todo: String::from("Todo 2"),
                info: String::from("My todo 2"),
                status: Status::InProgress,
                project_id: None,
                project: None,
                due_date: None,
                created_at: Local::now().date_naive(),
                completed_at: None,
            },
            Todo {
                id: 3,
                todo: String::from("Todo 3"),
                info: String::from("My todo 3"),
                status: Status::Done,
                project_id: None,
                project: None,
                due_date: None,
                created_at: Local::now().date_naive(),
                completed_at: None,
            },
        ]);

        todo_table.state.select(Some(0));
        todo_table.toggle_status(todo_table.state.selected().unwrap());
        assert_eq!(
            todo_table.items[todo_table.state.selected().unwrap()].status,
            Status::InProgress
        );

        todo_table.state.select(Some(1));
        todo_table.toggle_status(todo_table.state.selected().unwrap());
        assert_eq!(
            todo_table.items[todo_table.state.selected().unwrap()].status,
            Status::Done
        );

        todo_table.state.select(Some(2));
        todo_table.toggle_status(todo_table.state.selected().unwrap());
        assert_eq!(
            todo_table.items[todo_table.state.selected().unwrap()].status,
            Status::ToDo
        );

        Ok(())
    }
}
