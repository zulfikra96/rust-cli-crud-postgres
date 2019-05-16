extern crate postgres;

use postgres::{Connection,TlsMode};

pub fn conn() -> Connection{
    let conn = Connection::connect("postgres://postgres:Billgates1996@localhost:5432/rust", TlsMode::None)
        .unwrap();
    return conn;
}