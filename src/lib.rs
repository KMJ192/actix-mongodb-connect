#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate mongodb;

use mongodb::Client;
use std::sync::Mutex;
pub type Mongo = Mutex<Client>;

mod db;
mod routes;

pub use db::model;
pub use routes::*;

// mongo+srv//id:password~~~~
pub const MONGO_URL: &str = env!("MONGODB_URL");
pub const SERVER: &str = "0.0.0.0:8010";