use ogm::{models::Movie, models::Person, Connection};
//use hello_macro::HelloMacro;
use ogm::models::Queryable;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect().await?;
    println!("{:?}", Movie::find(&conn, 1).await?);
    println!("{:?}", Movie::find(&conn, 5).await?);
    //Movie::hello_macro();
    println!("{:?}", Person::find(&conn, 3).await?); 
    // ugh how the hell do these things work :) conn.end();
    Ok(())
}
