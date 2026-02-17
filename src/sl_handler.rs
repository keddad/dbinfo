use rusqlite::{Connection, Result};
use crate::models::{Table, Index, View, DatabaseInfo};

pub fn fetch_tables(conn: &Connection) -> Result<Vec<Table>> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")?;
    let table_names: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    let mut tables = Vec::new();
    for name in table_names {
        let row_count: u64 = conn.query_row(&format!(r#"SELECT count(*) FROM "{}""#, name), [], |row| row.get(0))?;

        let mut col_stmt = conn.prepare(&format!(r#"PRAGMA table_info("{}")"#, name))?;
        let columns: Vec<(String, String)> = col_stmt.query_map([], |row| {
            Ok((row.get(1)?, row.get(2)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

        tables.push(Table {
            name,
            size_bytes: 0,
            row_count,
            columns,
        });
    }
    Ok(tables)
}

pub fn fetch_indexes(conn: &Connection) -> Result<Vec<Index>> {
    let mut stmt = conn.prepare("SELECT name, tbl_name FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%'")?;
    let rows: Vec<(String, String)> = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(rows.into_iter().map(|(name, table_name)| Index {
        name,
        table_name,
        index_size: 0,
        index_scans: 0,
    }).collect())
}

pub fn fetch_views(conn: &Connection) -> Result<Vec<View>> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='view'")?;
    let names: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(names.into_iter().map(|name| View {
        name,
        size: None,
    }).collect())
}

pub fn fetch_sqlite_info(conn: &Connection) -> Result<DatabaseInfo> {
    let tables = fetch_tables(conn)?;
    let indexes = fetch_indexes(conn)?;
    let views = fetch_views(conn)?;

    Ok(DatabaseInfo {
        tables,
        indexes,
        views,
    })
}