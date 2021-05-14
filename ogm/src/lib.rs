//extern crate field_types;
//use field_types::{FieldName, FieldType};

//use std::stream::Stream;
use std::env;
use std::convert::TryFrom;
//use std::convert::TryInto;
use std::iter::FromIterator;

//use tokio::stream::Stream;

use bolt_client::*;
use bolt_proto::{message::*, version::*, Message, message::Record};

use tokio::io::BufStream;
use tokio_util::compat::*;

//use typename::TypeName;
pub mod models;

use std::cell::RefCell;

#[derive(Debug)]
//#[derive(Clone)]
pub struct Connection {
    client : RefCell<Client<Compat<BufStream<bolt_client::Stream>>>>
}

impl Connection {
    pub async fn run(&self, query: &str, meta: Metadata) -> Result<(Message, Vec<Record>), Box<dyn std::error::Error>> {
        self.client.borrow_mut().run_with_metadata(query, None, None).await?;
        let (response, records) = self.client.borrow_mut().pull(Some(meta.clone())).await?;
        
        // is it a bad idea to do this every time??
        self.client.borrow_mut().reset().await?;

        Ok((response, records))
    }

    pub async fn end(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.borrow_mut().goodbye().await?;
        Ok(())
    }

pub async fn connect() -> Result<Connection, Box<dyn std::error::Error>> {
    
    //println!("{:?}", MovieFieldName::Tagline.name());

    /*let test_movie = models::Movie {
        tagline: String::from("pls"),
        title: String::from("work"),
        released: 2021,
    };

    println!("{}", test_movie.released.type_name_of());
    *///println!("{}", type_name_of_val(&test_movie.released));
    //println!("{:?}", MovieFieldType::Tagline);
    //println!("{:?}", MovieFieldType::Tagline(String::from("mmhmm")));

    println!("connecting...");
    let stream = Stream::connect(
        env::var("BOLT_ADDR")?, env::var("BOLT_DOMAIN").ok()).await?;
    let stream = BufStream::new(stream).compat();
    
    //let mut result = Client::new(stream, &[V4_1, V4_0, 0, 0]).await;
    let result = Client::new(stream, &[V4_1, V4_0, 0, 0]).await;
    let mut client = result.unwrap();

    let response: Message = client.hello(
        Some(Metadata::from_iter(vec![
            ("user_agent", "my-client-name/1.0"),
            ("scheme", "basic"),
            ("principal", &env::var("BOLT_USER")?),
            ("credentials", &env::var("BOLT_PASS")?),
        ]))).await?;
    assert!(Success::try_from(response).is_ok());
    println!("yay!");

    Ok(Connection { client: RefCell::new(client) })

    //println!("{:?}", models::Movie::find_one(client).await?);

    // TODO: figure out how to continuously pull until you're sure you have all records? I guess
    /*let pull_meta = Metadata::from_iter(vec![("n", 200)]);

    client.run_with_metadata("MATCH (n:Movie { released: 1992 }) RETURN n;", None, None).await?;
    let (response, records) = client.pull(Some(pull_meta.clone())).await?;
    println!("{:?}", response);
    
    for node in records {
        let movie = Node::try_from(node.fields()[0].clone())?;
        let movie_struct = models::Movie {
            title : String::try_from(movie.properties().get("title").unwrap().clone())?,
            tagline : String::try_from(movie.properties().get("tagline").unwrap().clone())?,
            released : i32::try_from(movie.properties().get("released").unwrap().clone())?,
        };
        println!("{:?}", movie_struct);
    }*/

    // really should call this eventually, but i can't figure oot how to use this for now soooooo
    // client.goodbye().await?;
    //Ok(())
}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
