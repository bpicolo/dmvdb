#![feature(box_syntax)]
#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate rocksdb;


use std::str;
use nickel::{Nickel, HttpRouter, JsonBody};
use nickel::status::StatusCode;
use rocksdb::{DB, Writable, IteratorMode};
use rustc_serialize::json::{encode, Json, ToJson};


#[derive(RustcDecodable, RustcEncodable)]
struct Datom {
    key: String,
    value: String
}


#[derive(RustcDecodable, RustcEncodable)]
struct TransactionLog {
    id: u64,
    time: u64
}

fn main() {
    let mut server = Nickel::new();
    server.post("/log", middleware! { |request, mut response|
        let db = DB::open_default("/tmp/rocksdb/transactions").unwrap();
        match request.json_as::<TransactionLog>() {
            Ok(transaction_log) => {
                let _ = db.put(
                    transaction_log.id.to_string().as_bytes(),
                    transaction_log.time.to_string().as_bytes()
                );
                ""
            }
            Err(e) => {
                response.set(StatusCode::BadRequest);
                ""
            }
        }
    });
    server.get("/log/latest", middleware! { |request, mut response|
        let db = DB::open_default("/tmp/rocksdb/transactions").unwrap();
        let mut iter = db.iterator(IteratorMode::End);  // Always iterates backward
        match iter.next() {
            Some((key, value)) => {
                let log = TransactionLog {
                    id: str::from_utf8(&*key).unwrap().parse::<u64>().unwrap(),
                    time:  str::from_utf8(&*value).unwrap().parse::<u64>().unwrap()
                };
                encode(&log).unwrap()
            }
            None => String::from("")
        }
    });

    // server.post("", middleware! { |request, response|
    //     let db = DB::open_default("/tmp/rocksdb/storage").unwrap();
    //     let datum = request.json_as::<Datom>().unwrap();
    //     db.put(datum.key.as_bytes(), datum.value.as_bytes());
    //     format!("")
    // });
    // server.get("/:key", middleware! { |request, response|
    //     let db = DB::open_default("/tmp/rocksdb/storage").unwrap();
    //     let key = request.param("key").unwrap().as_bytes();
    //     match db.get(key) {
    //         Ok(Some(value)) => String::from(value.to_utf8().unwrap()),
    //         Ok(None) => format!(""),
    //         Err(e) => format!(""),
    //     }
    // });

    server.listen("127.0.0.1:3000");
}
