extern crate rand;

use self::rand::Rng;

struct ConnectionPool<F, T> where F: Fn() -> T {
    create_conn: F,
    pool: Vec<T>
}


impl<F, T> ConnectionPool<F, T> where F: Fn() -> T {
    fn new(create_conn: F) -> Self {
        let pool = vec![];
        ConnectionPool { create_conn: create_conn, pool: pool }
    }

    fn borrow_conn(&mut self) -> Option<T> {
        if self.pool.len() == 0 {
            println!("Creating new connection in pool");
            self.pool.push((self.create_conn)())
        } else {
            println!("Returning existing from pool");
        }
        self.pool.pop()
    }

    fn return_conn(&mut self, conn: T) {
        self.pool.push(conn);
    }
}


struct OracleConnection;

impl OracleConnection {
    fn new() -> OracleConnection {
        OracleConnection
    }
    fn query(&self, sql: &str) {
        println!("Running Oracle query: {}", sql)
    }
    fn close(&self) {
        println!("Closing an Oracle connection")
    }
}


impl Drop for OracleConnection {
    fn drop(&mut self) {
        self.close();
        println!("Dropping!");
    }
}


fn main() {
    let mut pool = ConnectionPool::new(OracleConnection::new);
    for _ in 0..10 {
        let conn = pool.borrow_conn().unwrap();
        conn.query("SELECT * FROM customer LIMIT 1");
        let mut rng = rand::thread_rng();
        if rng.gen() {
            pool.return_conn(conn);
        }
    }
}
