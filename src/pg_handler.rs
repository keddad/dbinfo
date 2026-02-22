use postgres::{Client, Error};
use crate::models::{Table, Index, View, DatabaseInfo};

pub fn fetch_tables(client: &mut Client) -> Result<Vec<Table>, Error> {
    let rows = client.query(
        "SELECT 
            t.relname AS table_name,
            pg_total_relation_size(t.oid) AS size_bytes,
            n.nspname as schema_name
        FROM pg_class t
        JOIN pg_namespace n ON n.oid = t.relnamespace
        WHERE t.relkind = 'r' 
        AND n.nspname NOT IN ('pg_catalog', 'information_schema')",
        &[],
    )?;

    let mut tables = Vec::new();
    for row in rows {
        let name: String = row.get("table_name");
        let size_bytes: i64 = row.get("size_bytes");
        let schema_name: String = row.get("schema_name");

        let count_row = client.query_one(
            &format!(r#"SELECT count(*) FROM "{}"."{}""#, schema_name, name),
            &[],
        )?;
        let row_count: i64 = count_row.get(0);

        let col_rows = client.query(
            "SELECT column_name, data_type 
             FROM information_schema.columns 
             WHERE table_name = $1 AND table_schema = $2",
            &[&name, &schema_name],
        )?;

        let columns = col_rows.iter().map(|r| (r.get(0), r.get(1))).collect();

        tables.push(Table {
            name,
            size_bytes: size_bytes as u64,
            row_count: row_count as u64,
            columns,
        });
    }
    Ok(tables)
}

pub fn fetch_indexes(client: &mut Client) -> Result<Vec<Index>, Error> {
    let rows = client.query(
        "SELECT
            i.relname as index_name,
            t.relname as table_name,
            pg_relation_size(i.oid) as index_size,
            s.idx_scan as index_scans
        FROM pg_class i
        JOIN pg_index idx ON i.oid = idx.indexrelid
        JOIN pg_class t ON t.oid = idx.indrelid
        JOIN pg_namespace n ON n.oid = i.relnamespace
        JOIN pg_stat_user_indexes s ON s.indexrelid = i.oid
        WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')",
        &[],
    )?;

    Ok(rows
        .iter()
        .map(|row| Index {
            name: row.get("index_name"),
            table_name: row.get("table_name"),
            index_size: row.get::<_, i64>("index_size") as u64,
            index_scans: row.get::<_, i64>("index_scans") as u64,
        })
        .collect())
}

pub fn fetch_views(client: &mut Client) -> Result<Vec<View>, Error> {
    let rows = client.query(
        "SELECT
            relname as name,
            pg_total_relation_size(c.oid) as size
        FROM pg_class c
        JOIN pg_namespace n ON n.oid = relnamespace
        WHERE relkind IN ('v', 'm') 
        AND nspname NOT IN ('pg_catalog', 'information_schema')",
        &[],
    )?;

    Ok(rows
        .iter()
        .map(|row| View {
            name: row.get("name"),
            size: Some(row.get::<_, i64>("size") as u64),
        })
        .collect())
}

pub fn fetch_postgres_info(client: &mut Client) -> Result<DatabaseInfo, Error> {
    let tables = fetch_tables(client)?;
    let indexes = fetch_indexes(client)?;
    let views = fetch_views(client)?;

            Ok(DatabaseInfo {
                tables,
                indexes,
                views,
            })
        }
        