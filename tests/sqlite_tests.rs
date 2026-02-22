use dbinfo::sl_handler;
use rusqlite::Connection;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            role TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).unwrap();

    conn.execute(
        "CREATE INDEX idx_users_username ON users(username)",
        [],
    ).unwrap();

    conn.execute(
        "CREATE VIEW user_roles AS 
        SELECT role, COUNT(*) as count FROM users GROUP BY role",
        [],
    ).unwrap();

    conn.execute(
        "INSERT INTO users (username, role) VALUES (?, ?)",
        ["Alice", "Admin"],
    ).unwrap();

    conn
}

#[test]
fn test_fetch_tables() {
    let conn = setup_test_db();
    let tables = sl_handler::fetch_tables(&conn).unwrap();
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].name, "users");
    assert_eq!(tables[0].row_count, 1);
    assert!(tables[0].columns.iter().any(|(name, _)| name == "username"));
}

#[test]
fn test_fetch_indexes() {
    let conn = setup_test_db();
    let indexes = sl_handler::fetch_indexes(&conn).unwrap();
    assert_eq!(indexes.len(), 1);
    assert_eq!(indexes[0].name, "idx_users_username");
    assert_eq!(indexes[0].table_name, "users");
}

#[test]
fn test_fetch_views() {
    let conn = setup_test_db();
    let views = sl_handler::fetch_views(&conn).unwrap();
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].name, "user_roles");
}

#[test]
fn test_fetch_sqlite_info() {
    let conn = setup_test_db();
    let info = sl_handler::fetch_sqlite_info(&conn).unwrap();
    assert_eq!(info.tables.len(), 1);
    assert_eq!(info.indexes.len(), 1);
    assert_eq!(info.views.len(), 1);
}
