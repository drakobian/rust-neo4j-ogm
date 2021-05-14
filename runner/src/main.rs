use ogm::{models::Movie, Connection};
//use hello_macro::HelloMacro;
use ogm::models::HelloMacro;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect().await?;
    println!("{:?}", Movie::find_one(&conn).await?);
    println!("{:?}", Movie::find_one(&conn).await?);
    Movie::hello_macro();

    // ugh how the hell do these things work :) conn.end();
    Ok(())
}
