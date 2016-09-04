extern crate unqlite;
extern crate serde_json;

use dbAdapter::dbConnector;
use dbAdapter::dbAdapter;
use std::io::prelude::*;
use std::fs::File;
use self::serde_json::Value;
use self::serde_json::builder::{ArrayBuilder, ObjectBuilder};
use std::env;
use std::str;

pub struct unqliteDB{
    handle: i64
}

impl unqliteDB {
    pub fn new() -> Self {
        unqliteDB { handle: 77}
    }
}

impl dbConnector for unqliteDB {
    fn open(&mut self) -> i64 {
        println!("opened");
        return 1;
    }

    fn get_name(&self) -> &'static str {
        "unqlite"
    }
}