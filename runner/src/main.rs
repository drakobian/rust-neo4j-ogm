extern crate dotenv;
use dotenv::dotenv;

use ogm::{models::{Movie, Person, Queryable}, Connection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let conn = Connection::connect().await?;
    println!("{:?}", Movie::find(&conn, 1).await?);
    println!("{:?}", Person::find(&conn, 3).await?);
    Ok(())
}
