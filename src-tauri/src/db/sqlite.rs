use rusqlite::Connection;

static mut DB_PATH: Option<String> = None;

pub fn set_path(db_path: &str) {
    unsafe {
        DB_PATH = Some(db_path.to_string());
    }
}

pub fn get_connection() -> Connection {
    let db_path = unsafe { DB_PATH.as_ref() }.expect("DB path not set");
    Connection::open(&db_path).expect("Failed to open database connection")
}
