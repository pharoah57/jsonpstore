
use dbAdapter::dbConnector;
use dbAdapter::dbAdapter;

pub struct sqliteDB{
    handle: i64
}

impl sqliteDB {
    pub fn new() -> Self {
        sqliteDB { handle: 77}
    }
}

impl dbConnector for sqliteDB {
    fn open(&mut self) -> i64 {
        println!("opened");
        return 1;
    }

    fn get_name(&self) -> &'static str {
        "sqlite"
    }
}