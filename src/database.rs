use rusqlite::Connection;

use crate::cli;

pub fn init() -> Connection {
    let conn = Connection::open("./tasks.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
        id          INTEGER PRIMARY KEY,
        title       TEXT NOT NULL,
        description TEXT,
        complete    BOOLEAN DEFAULT 0
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
        UPDATE tasks SET complete = 1 WHERE id = ?1",
        (id,),
    )
    .unwrap();
}

pub fn delete_task(conn: &Connection, id: i32) {
    conn.execute(
        "
    DELETE tasks WEHRE id = ?1",
        (id,),
    )
    .unwrap();
}

pub fn get_max_id(conn: &Connection) -> i32 {
    let max_id: i32 = conn
        .query_row(
            "
    SELECT COALESCE(MAX(ID), 0) FROM tasks",
            (),
            |r| r.get(0),
        )
        .unwrap();

    max_id
}
