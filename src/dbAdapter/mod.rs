pub use self::dbunqlite::*;
pub use self::dbsqlite::*;

pub mod dbsqlite;
pub mod dbunqlite;

pub struct dbAdapter {
    pub connection: &'static str,
    dbtype: &'static str
}

pub struct dbBuilder {
    connection: &'static str,
    dbtype: &'static str,
    username: &'static str,
    password: &'static str
}

pub trait dbConnector {
   fn open(&mut self) -> i64;
   fn get_name(&self) -> &'static str;
}

impl dbBuilder {
    pub fn new(dbType: &'static str) -> dbBuilder {
        dbBuilder { connection: "", dbtype: dbType, username: "", password: ""}
    }

    fn connection(&mut self, connection: &'static str) -> &mut dbBuilder {
        self.connection = connection;
        self
    }

    fn dbtype(&mut self, dbtype: &'static str) -> &mut dbBuilder {
        self.dbtype = dbtype;
        self
    }

    fn username(&mut self, username: &'static str) -> &mut dbBuilder {
        self.username = username;
        self
    }

    fn password(&mut self, password: &'static str) -> &mut dbBuilder {
        self.password = password;
        self
    }

    pub fn finalize(&mut self) -> Box<dbConnector> {
        match self.dbtype {
            "sqlite" => return Box::new(sqliteDB::new()),
            "unqlite" => return Box::new(unqliteDB::new()),
            _ => return Box::new(unqliteDB::new())
        }
    }
}