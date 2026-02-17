use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Table {
    pub name: String,
    pub size_bytes: u64,
    pub row_count: u64,
    pub columns: Vec<(String, String)>,
}

#[derive(Debug, Serialize)]
pub struct Index {
    pub name: String,
    pub table_name: String,
    pub index_size: u64,
    pub index_scans: u64,
}

#[derive(Debug, Serialize)]
pub struct View {
    pub name: String,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct DatabaseInfo {
    pub tables: Vec<Table>,
    pub indexes: Vec<Index>,
    pub views: Vec<View>,
}

pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
}

pub fn get_type_by_uri(uri: &str) -> Option<DatabaseType> {
    if uri.starts_with("postgres://") || uri.starts_with("postgresql://") {
        Some(DatabaseType::Postgres)
    } else if uri.starts_with("mysql://") {
        Some(DatabaseType::Mysql)
    } else if uri.contains(".db") || uri.contains(".sqlite") || uri.starts_with("file:") {
        Some(DatabaseType::Sqlite)
    } else {
        None
    }
}
