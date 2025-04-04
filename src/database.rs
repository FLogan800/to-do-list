use crate::cli;
use crate::task;
use rusqlite::Connection;

pub fn init() -> Connection {
    let conn = Connection::open("./tasks.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
        id          INTEGER PRIMARY KEY,
        title       TEXT NOT NULL,
        description TEXT,
        is_complete    BOOLEAN DEFAULT 0
        )",
        (),
    )
    .unwrap();

    conn
}

pub fn insert_task(conn: &Connection, task: &cli::NewTask) {
    let id = get_max_id(conn) + 1;

    conn.execute(
        "
    INSERT INTO tasks (id, title, description) VALUES (?1, ?2, ?3)",
        (id, task.title.clone(), task.description.clone()),
    )
    .unwrap();
}

pub fn complete_task(conn: &Connection, id: i32) {
    conn.execute(
        "
        UPDATE tasks SET is_complete = 1 WHERE id = ?1",
        (id,),
    )
    .unwrap();
}

pub fn delete_task(conn: &Connection, id: i32) {
    conn.execute(
        "
    DELETE FROM tasks WHERE id = ?1",
        (id,),
    )
    .unwrap();
}

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

pub fn fetch_tasks(conn: &Connection) -> Vec<task::Task> {
    let mut tasks = conn.prepare("SELECT * FROM tasks").unwrap();

    let task_iter = tasks
        .query_map([], |row| {
            Ok(task::Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                is_complete: row.get(3)?,
            })
        })
        .unwrap();

    let mut task_vec = Vec::<task::Task>::new();

    for task in task_iter {
        task_vec.push(task.unwrap());
    }

    task_vec
}
