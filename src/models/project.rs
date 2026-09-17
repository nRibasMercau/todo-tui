use chrono::NaiveDate;

pub type ProjectId = i64;

// Db record
// and UI representation
#[derive(Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub archived: bool,
    pub created_at: NaiveDate,
}

// Info for creating a new project
#[derive(Debug)]
pub struct NewProject {
    pub name: String,
    pub archived: bool,
}

impl NewProject {
    pub fn new(name: String) -> Self {
        Self {
            name,
            archived: false,
        }
    }
}
