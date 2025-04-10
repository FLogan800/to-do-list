use crate::cli;
use crate::task;
use chrono::NaiveDate;
use rusqlite::Connection;

// Initilezes a database conenction and ensures a table exists
pub fn init() -> Connection {
    let conn = Connection::open("./tasks.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
        id          INTEGER PRIMARY KEY,
        title       TEXT NOT NULL,
        description TEXT,
        priority    INTEGER DEFAULT 1,
        is_complete BOOLEAN DEFAULT 0,
        due_date    DATE
        )",
        (),
    )
    .unwrap();

    conn
}

// Adds a new task into the database and assigns it a unique ID
pub fn insert_task(conn: &Connection, task: &cli::NewTask) {
    let id = get_max_id(conn) + 1;

    // Parse the due date string in the format "mm/dd/yyyy"
    let due_date: Option<String> = match &task.due_date {
        Some(date_str) => match NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
            Ok(due_date) => Some(due_date.format("%m/%d/%Y").to_string()),
            Err(_) => panic!("Invalid date format: {}", date_str),
        },
        None => None,
    };

    conn.execute(
        "
    INSERT INTO tasks (id, title, description, priority, due_date) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            id,
            task.title.clone(),
            task.description.clone(),
            task.priority,
            due_date,
        ),
    )
    .unwrap();
}

// Marks a task complete by ID
pub fn complete_task(conn: &Connection, id: i32) {
    conn.execute(
        "
        UPDATE tasks SET is_complete = 1 WHERE id = ?1",
        (id,),
    )
    .unwrap();
}

// Marks a task complete by ID
pub fn uncomplete_task(conn: &Connection, id: i32) {
    conn.execute(
        "
        UPDATE tasks SET is_complete = 0 WHERE id = ?1",
        (id,),
    )
    .unwrap();
}

// Completely removes a task from the database
pub fn delete_task(conn: &Connection, id: i32) {
    conn.execute(
        "
    DELETE FROM tasks WHERE id = ?1",
        (id,),
    )
    .unwrap();
}

// Gets the highest task ID from the database
pub fn get_max_id(conn: &Connection) -> i32 {
    let max_id: i32 = conn
        .query_row(
            "
    SELECT COALESCE(MAX(id), 0) FROM tasks",
            (),
            |r| r.get(0),
        )
        .unwrap();

    max_id
}

// Fetches all tasks from the database and returns them in a vector
pub fn fetch_tasks(conn: &Connection) -> Vec<task::Task> {
    let mut tasks = conn.prepare("SELECT * FROM tasks").unwrap();

    let task_iter = tasks
        .query_map([], |row| {
            Ok(task::Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                is_complete: row.get(4)?,
                due_date: {
                    let date_str: Option<String> = row.get(5)?;

                    match date_str {
                        Some(date_str) => match NaiveDate::parse_from_str(&date_str, "%m/%d/%Y") {
                            Ok(due_date) => Some(due_date),
                            Err(_) => None,
                        },
                        None => None,
                    }
                },
            })
        })
        .unwrap();

    let mut task_vec = Vec::<task::Task>::new();

    for task in task_iter {
        task_vec.push(task.unwrap());
    }

    task_vec
}
