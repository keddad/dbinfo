use std::fmt;

pub enum DatabaseType {
    Postgres
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseType::Postgres => write!(f, "Postgres"),
        }
    }
}

pub fn get_type_by_uri(uri: &String) -> Option<DatabaseType> {
    if uri.starts_with("postgres") {
        Some(DatabaseType::Postgres)
    } else {
        None
    }
}