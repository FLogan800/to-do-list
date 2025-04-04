use rusqlite::Connection;

use crate::task;

pub fn init() -> Connection {
    let conn = Connection::open("./tasks.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
        id          INTEGER PRIMARY KEY,
        title       TEXT NOT NULL,
        description TEXT,
        complete    BOOLEAN
        )",
        (),
    )
    .unwrap();

    conn
}

pub fn insert_task(conn: &Connection, task: task::Task) {
    conn.execute(
        "
    INSERT INTO tasks (id, title, description, complete) VALUES (?1, ?2, ?3, ?4)",
        (task.id, task.title, task.description, task.complete),
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
    SELECT COUNT(1) FROM tasks",
            (),
        |r| r.get(0)
        )
        .unwrap();

    max_id
}
