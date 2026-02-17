use std::env;

mod models;
mod pg_handler;
mod my_handler;
mod sl_handler;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let uri = args.get(1).ok_or("Can't get URI from args")?;

    let database = models::get_type_by_uri(uri)
        .ok_or("Unsupported database or broken connection string")?;

    let info = match database {
        models::DatabaseType::Postgres => {
            let mut client = postgres::Client::connect(uri, postgres::NoTls)
                .map_err(|e: postgres::Error| e.to_string())?;
            pg_handler::fetch_postgres_info(&mut client).map_err(|e| e.to_string())?
        }
        models::DatabaseType::Mysql => {
            let mut conn = mysql::Conn::new(uri.as_str()).map_err(|e: mysql::Error| e.to_string())?;
            my_handler::fetch_mysql_info(&mut conn).map_err(|e| e.to_string())?
        }
        models::DatabaseType::Sqlite => {
            let conn = rusqlite::Connection::open(uri).map_err(|e: rusqlite::Error| e.to_string())?;
            sl_handler::fetch_sqlite_info(&conn).map_err(|e| e.to_string())?
        }
    };

    let json = serde_json::to_string_pretty(&info).map_err(|e| e.to_string())?;
    println!("{}", json);

    Ok(())
}