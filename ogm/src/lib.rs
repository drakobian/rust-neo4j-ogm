use std::env;
use std::convert::TryFrom;
use std::iter::FromIterator;

use bolt_client::*;
use bolt_proto::{message::*, version::*, Message, message::Record};

use tokio::io::BufStream;
use tokio_util::compat::*;

pub mod models;

use std::cell::RefCell;

#[derive(Debug)]
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
    
        println!("connecting...");
        let stream = Stream::connect(
            env::var("BOLT_ADDR")?, env::var("BOLT_DOMAIN").ok()).await?;
        let stream = BufStream::new(stream).compat();
    
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

        Ok(Connection { client: RefCell::new(client) })
    
    }
}
