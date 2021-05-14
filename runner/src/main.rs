use ogm::{models::Movie, Connection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect().await?;
    println!("{:?}", Movie::find_one(&conn).await?);
    println!("{:?}", Movie::find_one(&conn).await?);

    // ugh how the hell do these things work :) conn.end();
    Ok(())
}
