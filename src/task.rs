use std::fmt;

use chrono::NaiveDate;

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub is_complete: bool,
    pub due_date: Option<NaiveDate>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nID: {}\nPriority: {}\nDescription: {}\nCompleted: {}\nDue Date: {}\n",
            self.title,
            self.id,
            self.priority,
            match &self.description {
                Some(description) => description,
                None => "",
            },
            self.is_complete,
            match self.due_date {
                Some(date) => date.format("%m/%d/%Y").to_string(),
                None => String::from("None"),
            },
        )
    }
}
