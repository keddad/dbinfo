use dbinfo::pg_handler;
use postgres::{Client, NoTls};
use testcontainers::runners::SyncRunner;
use testcontainers_modules::postgres::Postgres;

#[test]
fn test_fetch_postgres_info() {
    let node = Postgres::default().start().unwrap();
    let host_port = node.get_host_port_ipv4(5432).unwrap();
    let connection_string = format!(
        "postgresql://postgres:postgres@localhost:{}/postgres",
        host_port
    );

    let mut client = Client::connect(&connection_string, NoTls).unwrap();

    // Setup schema
    client.execute(
        "CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) NOT NULL,
            role VARCHAR(20)
        )",
        &[],
    ).unwrap();

    client.execute(
        "CREATE INDEX idx_users_username ON users(username)",
        &[],
    ).unwrap();

    client.execute(
        "CREATE VIEW user_roles AS 
        SELECT role, COUNT(*) FROM users GROUP BY role",
        &[],
    ).unwrap();

    client.execute(
        "INSERT INTO users (username, role) VALUES ('Alice', 'Admin')",
        &[],
    ).unwrap();

    let info = pg_handler::fetch_postgres_info(&mut client).unwrap();

    assert!(info.tables.iter().any(|t| t.name == "users"));
    assert!(info.indexes.iter().any(|i| i.name == "idx_users_username"));
    assert!(info.views.iter().any(|v| v.name == "user_roles"));

    let users_table = info.tables.iter().find(|t| t.name == "users").unwrap();
    assert_eq!(users_table.row_count, 1);
    assert!(users_table.columns.iter().any(|(name, _)| name == "username"));
}
