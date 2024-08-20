use super::sqlite;

pub fn apply_migrations() {
    let conn = sqlite::get_connection();
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS plan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            start_time INTEGER NOT NULL,
            end_time INTEGER NOT NULL,

            source TEXT,
            external_id TEXT,

            UNIQUE(source, external_id)
        );

        CREATE TABLE IF NOT EXISTS work_session (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time INTEGER NOT NULL,
            end_time INTEGER,
            status TEXT NOT NULL CHECK(status IN ('open', 'close'))
        );

        CREATE TABLE IF NOT EXISTS window_activity (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            class TEXT NOT NULL,
            execute_binary TEXT,

            FOREIGN KEY(id) REFERENCES activity(id)
        );
        CREATE TABLE IF NOT EXISTS browser_activity (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            url TEXT NOT NULL,

            FOREIGN KEY(id) REFERENCES activity(id)
        );
        CREATE TABLE IF NOT EXISTS activity (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL CHECK(type IN ('window', 'browser')),
            category TEXT
        );

        CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time INTEGER,
            end_time INTEGER,
            activity_id TEXT NOT NULL,
            
            FOREIGN KEY(activity_id) REFERENCES activity(id)
        );

        CREATE VIEW IF NOT EXISTS activity_log AS
            SELECT activity.id AS name, 
                log.start_time, 
                log.end_time, 
                activity.category AS category
            FROM activity
            JOIN log ON activity.id = log.activity_id;
    ",
    )
    .unwrap();
}
