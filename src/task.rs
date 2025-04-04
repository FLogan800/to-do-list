use std::fmt;

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub complete: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nID: {}\nDescription: {:?}\nComplete: {}\n",
            self.title,
            self.id,
            self.description,
            self.complete
        )
    }
}
