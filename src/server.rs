use mysql::Conn;
use mysql::Opts;
use mysql::error::Result as MyResult;
use mysql::QueryResult;

pub struct Server {
    name: String,
    conn: Conn
}

impl Server {
    pub fn new<O>(opts : O, name : String) -> Result<Server, ()> 
    where O : Into<Opts> {
        Ok(Server {
            name,
            conn: Conn::new(opts.into()).expect("could not create connection")
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn conn(&mut self) -> &mut Conn {
        &mut self.conn
    }

    pub fn query (&mut self, q: &str) -> MyResult<QueryResult> {
        println!("[{}] Query: {}", self.get_name(), q);
        self.conn().query(q)
    }
}