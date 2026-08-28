#[derive(Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub archive: bool,
}

#[derive(Debug)]
pub struct NewProject {
    name: String,
    archive: bool,
}

impl NewProject {
    pub fn new(name: String) -> Self {
        Self {
            name,
            archive: false,
        }
    }
}
