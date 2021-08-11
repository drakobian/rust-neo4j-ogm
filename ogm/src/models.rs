use bolt_client::*;
use bolt_proto::{value::*};

use std::iter::FromIterator;
use std::convert::TryFrom;

use super::Connection;

use queryable_macro::{QNode, QRelationship};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait QNode {
    type Entity;

    async fn find(conn: &Connection, n: i32) -> Result<Vec<Self::Entity>, Box<dyn std::error::Error>>;
    fn from_node(node: Node) -> Option<Self::Entity>;
}

#[async_trait(?Send)]
pub trait QRelationship {
    type Entity;

    async fn find(conn: &Connection, n: i32) -> Result<Vec<Self::Entity>, Box<dyn std::error::Error>>;
    fn from_rel(rel: Relationship) -> Option<Self::Entity>;
}

#[derive(QNode)]
#[derive(Debug)]
pub struct Movie {
    pub tagline: String,
    pub title: String,
    pub released: i32,
}

#[derive(QNode)]
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub born: i32,
}

#[derive(QRelationship)]
#[derive(Debug)]
pub struct ACTED_IN {
    pub roles: Vec::<String>
}

