use dbinfo::models::{get_type_by_uri, DatabaseType};

#[test]
fn test_get_type_by_uri() {
    assert!(matches!(get_type_by_uri("postgres://user:pass@localhost:5432/db"), Some(DatabaseType::Postgres)));
    assert!(matches!(get_type_by_uri("postgresql://user:pass@localhost:5432/db"), Some(DatabaseType::Postgres)));
    assert!(matches!(get_type_by_uri("mysql://user:pass@localhost:3306/db"), Some(DatabaseType::Mysql)));
    assert!(matches!(get_type_by_uri("test.db"), Some(DatabaseType::Sqlite)));
    assert!(matches!(get_type_by_uri("test.sqlite"), Some(DatabaseType::Sqlite)));
    assert!(matches!(get_type_by_uri("file:test.db"), Some(DatabaseType::Sqlite)));
    assert!(get_type_by_uri("somethingelse").is_none());
}
