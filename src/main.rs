extern crate rustc_serialize;
extern crate rocksdb;
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};
use nickel::status::StatusCode::NotFound;
use rocksdb::{DB, Writable, IteratorMode};


#[derive(RustcDecodable, RustcEncodable)]
struct Datom {
    key: String,
    value: String
}

fn main() {
    let mut server = Nickel::new();
    server.post("/", middleware! { |request, response|
        let db = DB::open_default("/tmp/rocksdb/storage").unwrap();
        let datum = request.json_as::<Datom>().unwrap();
        db.put(datum.key.as_bytes(), datum.value.as_bytes());
        format!("")
    });
    server.get("/:key", middleware! { |request, response|
        let db = DB::open_default("/tmp/rocksdb/storage").unwrap();
        let key = request.param("key").unwrap().as_bytes();
        match db.get(key) {
            Ok(Some(value)) => String::from(value.to_utf8().unwrap()),
            Ok(None) => format!(""),
            Err(e) => format!(""),
        }
    });

    server.listen("127.0.0.1:3000");
}
