use std::fmt;

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub is_complete: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nID: {}\nDescription: {}\nIs Complete: {}\n",
            self.title,
            self.id,
            match &self.description {
                Some(description) => description,
                None => "",
            },
            self.is_complete
        )
    }
}
