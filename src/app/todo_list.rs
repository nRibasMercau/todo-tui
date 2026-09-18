use crate::models::todo::{Status, Todo};
use chrono::NaiveDate;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<Todo>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum TodoListError {
    TodoNotFound,
}

impl
    FromIterator<(
        i64,
        String,
        String,
        Status,
        Option<i64>,
        Option<String>,
        Option<NaiveDate>,
        NaiveDate,
        Option<NaiveDate>,
    )> for TodoList
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<
            Item = (
                i64,
                String,
                String,
                Status,
                Option<i64>,
                Option<String>,
                Option<NaiveDate>,
                NaiveDate,
                Option<NaiveDate>,
            ),
        >,
    {
        let items: Vec<Todo> = iter
            .into_iter()
            .map(
                |(
                    id,
                    todo,
                    info,
                    status,
                    project_id,
                    project,
                    due_date,
                    created_at,
                    completed_at,
                )| Todo {
                    id,
                    todo,
                    info,
                    status,
                    project_id,
                    project,
                    due_date,
                    created_at,
                    completed_at,
                },
            )
            .collect();

        // State
        // By default, the first item of the list will be selected
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { items, state }
    }
}

impl TodoList {
    pub fn new(items: Vec<Todo>) -> Self {
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

    pub fn add_todo(&mut self, todo: Todo) {
        self.items.push(todo);
    }

    pub fn replace_todo(&mut self, todo_item: Todo) -> Result<(), TodoListError> {
        match self.items.iter_mut().find(|i| i.id == todo_item.id) {
            Some(item) => {
                *item = todo_item;
                Ok(())
            }
            None => Err(TodoListError::TodoNotFound),
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items[i].status = self.items[i].status.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::todo::{Status, Todo};
    use chrono::Local;
    use color_eyre::Result;

    #[test]
    fn todo_list_select_next() -> Result<()> {
        let mut todo_list = TodoList::new(vec![
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

        todo_list.state.select(Some(0));

        todo_list.select_next();
        assert_eq!(todo_list.state.selected(), Some(1));

        todo_list.select_next();
        assert_eq!(todo_list.state.selected(), Some(2));

        todo_list.select_next();
        assert_eq!(todo_list.state.selected(), Some(3));

        Ok(())
    }

    #[test]
    fn todo_list_select_previous() -> Result<()> {
        let mut todo_list = TodoList::new(vec![
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

        todo_list.state.select(Some(2));

        todo_list.select_previous();
        assert_eq!(todo_list.state.selected(), Some(1));

        todo_list.select_previous();
        assert_eq!(todo_list.state.selected(), Some(0));

        todo_list.select_previous();
        assert_eq!(todo_list.state.selected(), Some(0));

        Ok(())
    }

    #[test]
    fn empty_list_select_next() -> Result<()> {
        let mut todo_list = TodoList::new(vec![]);

        assert_eq!(todo_list.state, ListState::default());

        todo_list.select_next();
        assert_eq!(todo_list.state.selected(), None);

        todo_list.select_next();
        assert_eq!(todo_list.state.selected(), None);

        Ok(())
    }

    #[test]
    fn empty_list_select_previous() -> Result<()> {
        let mut todo_list = TodoList::new(vec![]);

        assert_eq!(todo_list.state, ListState::default());

        todo_list.select_previous();
        assert_eq!(todo_list.state.selected(), None);

        todo_list.select_previous();
        assert_eq!(todo_list.state.selected(), None);

        Ok(())
    }
}
