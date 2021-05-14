use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
#[derive(Debug)]
pub struct Movie {
    pub tagline: String,
    pub title: String,
    pub released: i32,
}

use bolt_client::*;
use bolt_proto::{value::*};

use tokio::io::BufStream;
use tokio_util::compat::*;

use std::iter::FromIterator;
use std::convert::TryFrom;

use super::Connection;

impl Movie {
    // todo: convert this to just a 'find' that takes an 'n' for number, and returns a Vec<Movie> 
    pub async fn find_one(conn: &Connection) -> Result<Movie, Box<dyn std::error::Error>> {
        let pull_meta = Metadata::from_iter(vec![("n", 1)]);
        let (_response, records) = conn.run("MATCH (n:Movie) RETURN n;", pull_meta).await?;
        let node = Node::try_from(records[0].fields()[0].clone())?;
        /*Ok(Movie {
            title : String::try_from(record.properties().get("title").unwrap().clone())?,
            tagline : String::try_from(record.properties().get("tagline").unwrap().clone())?,
            released : i32::try_from(record.properties().get("released").unwrap().clone())?,
        })*/
        Ok(Movie::from_node(node).unwrap())
    }

    fn from_node(node: Node) -> Option<Movie> {
        // todo: investigate using an enum around String, i32 ...
        // and a method here to wrap around try_from that returns ...that same enum?
        // and then all my structs need to have fields that are that enum instead? Hmm
        // sounds like it could work....................................
        Some(Movie {
        title : String::try_from(node.properties().get("title").unwrap().clone()).ok()?,
        tagline : String::try_from(node.properties().get("tagline").unwrap().clone()).ok()?,
        released : i32::try_from(node.properties().get("released").unwrap().clone()).ok()?,
        })

        //Some(Movie { title, tagline, released })
    }
}
/*impl Movie {
    pub async fn find_one(mut client:  -> Result<Movie, Box<dyn std::error::Error>>  { 
        let pull_meta = Metadata::from_iter(vec![("n", 1)]);

        client.run_with_metadata("MATCH (n:Movie) RETURN n;", None, None).await?;
        let (_response, records) = client.pull(Some(pull_meta.clone())).await?;
        //println!("{:?}", response);
     
        let movie = Node::try_from(records[0].fields()[0].clone())?;
        Ok(Movie {
            title : String::try_from(movie.properties().get("title").unwrap().clone())?,
            tagline : String::try_from(movie.properties().get("tagline").unwrap().clone())?,
            released : i32::try_from(movie.properties().get("released").unwrap().clone())?,
        })
    }
}*/
