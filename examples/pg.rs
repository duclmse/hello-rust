use postgres::{Client, NoTls};
use std::result::Result;
use tokio_postgres::Error;

#[cfg(test)]
fn test_db_connection() -> std::io::Result<()> {
    let mut client = Client::connect("host=localhost user=postgres password=1", NoTls)?;

    let created = client.batch_execute(
        "CREATE TABLE person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )",
    );
    match created {
        Ok(_) => println!("created"),
        Err(err) =>  println!("error {err}"),
    }

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    )?;

    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    Ok(())
}
