use dbinfo::my_handler;
use mysql::prelude::*;
use mysql::Conn;
use testcontainers::runners::SyncRunner;
use testcontainers_modules::mysql::Mysql;

#[test]
fn test_fetch_mysql_info() {
    let node = Mysql::default().start().unwrap();
    let host_port = node.get_host_port_ipv4(3306).unwrap();
    
    let connection_string = format!(
        "mysql://root@localhost:{}/test",
        host_port
    );

    let mut conn = Conn::new(connection_string.as_str()).unwrap();

    // Setup schema
    conn.query_drop(
        "CREATE TABLE users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(50) NOT NULL,
            role VARCHAR(20)
        )"
    ).unwrap();

    conn.query_drop(
        "CREATE INDEX idx_users_username ON users(username)"
    ).unwrap();

    conn.query_drop(
        "CREATE VIEW user_roles AS 
        SELECT role, COUNT(*) as count FROM users GROUP BY role"
    ).unwrap();

    conn.query_drop(
        "INSERT INTO users (username, role) VALUES ('Alice', 'Admin')"
    ).unwrap();

    let info = my_handler::fetch_mysql_info(&mut conn).unwrap();

    assert!(info.tables.iter().any(|t| t.name == "users"));
    assert!(info.indexes.iter().any(|i| i.name == "idx_users_username"));
    assert!(info.views.iter().any(|v| v.name == "user_roles"));

    let users_table = info.tables.iter().find(|t| t.name == "users").unwrap();
    assert_eq!(users_table.row_count, 1);
    assert!(users_table.columns.iter().any(|(name, _)| name == "username"));
}
