extern crate dotenv;
use dotenv::dotenv;

use ogm::Connection;

use std::fs;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("migrating!");
    let conn = Connection::connect().await?;

    let query = "MATCH (n) RETURN distinct labels(n)";
    let (response, records) = conn.run(query).await?;

    let mut labels = vec![];
    for record in records {
        labels.append(&mut Vec::<String>::try_from(record.fields()[0].clone())?);
    }

    let query = "MATCH ()-[r]-() RETURN distinct type(r)";
    let (response, records) = conn.run(query).await?;

    let mut types = vec![];
    for record in records {
        types.push(String::try_from(record.fields()[0].clone())?)
    }

    let mut data = fs::read("models_template.txt").expect("Unable to read file");
    
    for label in labels {
        let query = format!("CALL apoc.meta.nodeTypeProperties({{labels: ['{}']}});", label);
        let (response, records) = conn.run(&query).await?;
        
        data.append(&mut String::from("#[derive(QNode)]\n").into_bytes());
        data.append(&mut String::from("#[derive(Debug)]\n").into_bytes());
        data.append(&mut format!("pub struct {} {{\n", label).into_bytes());

        for record in records {
            let field_name = String::try_from(record.fields()[2].clone())?;
            data.append(&mut format!("  pub {}: ", field_name).into_bytes());

            let field_types = Vec::<String>::try_from(record.fields()[3].clone())?;
            let mut field_type = String::from("String");
            if field_types[0] == String::from("Long") {
                field_type = String::from("i32");
            }
            data.append(&mut format!("{},\n", field_type).into_bytes());
        }

        data.append(&mut String::from("}\n\n").into_bytes());
    }
    
    fs::write("../ogm/src/models.rs", data).expect("Unable to write file");

    Ok(())
}