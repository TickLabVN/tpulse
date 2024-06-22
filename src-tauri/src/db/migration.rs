use super::sqlite;

pub fn apply_migrations() {
    let conn = sqlite::get_connection();
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            day DATE NOT NULL,
            start_time INTEGER,
            end_time INTEGER,
            task_name TEXT NOT NULL,
            category_tag TEXT,
            priority_tag TEXT CHECK(priority_tag IN ('high', 'medium', 'low'))
        );
        CREATE TABLE IF NOT EXISTS activity (
            identifier TEXT PRIMARY KEY,
            activity_tag TEXT,
            category_tag TEXT
        );
        CREATE TABLE IF NOT EXISTS log (
            start_time INTEGER PRIMARY KEY,
            end_time INTEGER,
            activity_identifier TEXT NOT NULL,
            task_id INTEGER,
            FOREIGN KEY(task_id) REFERENCES tasks(id)
            FOREIGN KEY(activity_identifier) REFERENCES activity(identifier)
        );
        CREATE VIEW IF NOT EXISTS activity_log AS
            SELECT activity.identifier AS name, 
                log.start_time, 
                log.end_time, 
                activity.category_tag, 
                log.task_id
            FROM activity 
            JOIN log ON activity.identifier = log.activity_identifier;
    ",
    )
    .unwrap();
}
