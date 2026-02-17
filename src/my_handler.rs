use mysql::prelude::*;
use mysql::{Conn, Error};
use crate::models::{Table, Index, View, DatabaseInfo};

pub fn fetch_tables(conn: &mut Conn) -> Result<Vec<Table>, Error> {
    let rows: Vec<(String, u64, String)> = conn.query(
        "SELECT 
            table_name,
            (data_length + index_length) as size_bytes,
            table_schema
        FROM information_schema.tables
        WHERE table_type = 'BASE TABLE'
        AND table_schema NOT IN ('information_schema', 'mysql', 'performance_schema', 'sys')",
    )?;

    let mut tables = Vec::new();
    for (name, size_bytes, schema) in rows {
        let row_count: u64 = conn.query_first(format!("SELECT COUNT(*) FROM `{}`.`{}`", schema, name))?
            .unwrap_or(0);

        let col_rows: Vec<(String, String)> = conn.query(format!(
            "SELECT column_name, data_type 
             FROM information_schema.columns 
             WHERE table_name = '{}' AND table_schema = '{}'",
            name, schema
        ))?;

        tables.push(Table {
            name,
            size_bytes,
            row_count,
            columns: col_rows,
        });
    }
    Ok(tables)
}

pub fn fetch_indexes(conn: &mut Conn) -> Result<Vec<Index>, Error> {
    let rows: Vec<(String, String)> = conn.query(
        "SELECT DISTINCT
            index_name,
            table_name
        FROM information_schema.statistics
        WHERE table_schema NOT IN ('information_schema', 'mysql', 'performance_schema', 'sys')",
    )?;

    Ok(rows.into_iter().map(|(name, table_name)| Index {
        name,
        table_name,
        index_size: 0,
        index_scans: 0,
    }).collect())
}

pub fn fetch_views(conn: &mut Conn) -> Result<Vec<View>, Error> {
    let names: Vec<String> = conn.query(
        "SELECT table_name
        FROM information_schema.views
        WHERE table_schema NOT IN ('information_schema', 'mysql', 'performance_schema', 'sys')",
    )?;

    Ok(names.into_iter().map(|name| View {
        name,
        size: None,
    }).collect())
}

pub fn fetch_mysql_info(conn: &mut Conn) -> Result<DatabaseInfo, Error> {
    let tables = fetch_tables(conn)?;
    let indexes = fetch_indexes(conn)?;
    let views = fetch_views(conn)?;

    Ok(DatabaseInfo {
        tables,
        indexes,
        views,
    })
}
