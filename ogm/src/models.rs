use bolt_client::*;
use bolt_proto::{value::*};

use std::iter::FromIterator;
use std::convert::TryFrom;

use super::Connection;

use queryable_macro::Queryable;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Queryable {
    type Entity;

    async fn find(conn: &Connection, n: i32) -> Result<Vec<Self::Entity>, Box<dyn std::error::Error>>;
    fn from_node(node: Node) -> Option<Self::Entity>;
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct Movie {
    pub tagline: String,
    pub title: String,
    pub released: i32,
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub born: i32,
}

