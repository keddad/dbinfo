use std::{env, time::Duration};

use postgres::{Client, NoTls};

mod dbinfo;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let uri = args.get(1).ok_or("Can't get URI from args")?;

    let database = dbinfo::get_type_by_uri(uri).ok_or("Unsupported database or broken connection string")?;

    println!("{}", database);

    let mut client = Client::connect(uri, NoTls).unwrap();

    client.is_valid(Duration::from_secs(5)).unwrap();

    Ok(())
}
