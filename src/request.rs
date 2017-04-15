pub use error::{Error, Result};
use url::Url;
use serde::Deserialize;
use serde_json::Value;
use std::marker::PhantomData;
use api::{HasValue, HasPagination};
use values::HasResponse;
use action::{Action, List, Get, Post, Delete};
use DigitalOcean;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Request<A, R> where A: Action {
    pub url: Url,
    pub body: Option<Value>,
    pub action: PhantomData<A>,
    pub response_type: PhantomData<R>,
}

impl<A, R> Request<A, R>
where A: Action {
    pub fn new(url: Url) -> Self {
        Request {
            url: url,
            body: None,
            action: PhantomData,
            response_type: PhantomData,
        }
    }
    pub fn body<'a>(&'a mut self, body: Value) -> &'a mut Self {
        self.body = Some(body);
        self
    }
    pub fn url<'a>(&'a mut self, url: Url) -> &'a mut Self {
        self.url = url;
        self
    }
}

pub trait Retrievable<T>: Sized
where T: Deserialize + Clone + HasResponse,
      T::Response: HasValue<Value=T> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<T>;
}

impl<V> Retrievable<Vec<V>> for Request<List, Vec<V>>
where Vec<V>: HasResponse,
      V: Deserialize + Clone,
      <Vec<V> as HasResponse>::Response: HasValue<Value=Vec<V>> + HasPagination {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<Vec<V>> {
        info!("Retrieving GET list.");
        let response: Vec<V> = instance.list(self)?;
        Ok(response)
    }
}

impl<V> Retrievable<V> for Request<Post, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<V> {
        info!("Retrieving GET.");
        let response = instance.post(self)?;
        Ok(response)
    }
}

impl<V> Retrievable<V> for Request<Get, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<V> {
        info!("Retrieving GET.");
        let response = instance.get(self)?;
        Ok(response)
    }
}

impl Retrievable<()> for Request<Delete, ()> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<()> {
        info!("Retrieving GET.");
        let response = instance.delete(self)?;
        Ok(response)
    }
}