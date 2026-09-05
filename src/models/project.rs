// Db record
// and UI representation
#[derive(Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub archive: bool,
}

// Info for creating a new project
#[derive(Debug)]
pub struct NewProject {
    pub name: String,
    pub archive: bool,
}

impl NewProject {
    pub fn new(name: String) -> Self {
        Self {
            name,
            archive: false,
        }
    }
}
