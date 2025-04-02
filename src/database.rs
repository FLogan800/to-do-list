use rusqlite::Connection;

use crate::task;

pub fn init() -> Connection {
    let conn = Connection::open("./tasks.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
        id          INTEGER PRIMARY KEY,
        title       TEXT NOT NULL,
        description TEXT
        )",
        (),
    )
    .unwrap();

    conn
}

pub fn insert_task(conn: Connection, task: task::Task) {
    conn.execute(
        "
    INSERT INTO tasks (id, title, description) VALUES (?1, ?2, ?3)",
        (task.id, task.title, task.description),
    )
    .unwrap();
}
