#![feature(box_syntax)]
#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate rocksdb;

use std::str;
use nickel::{Nickel, HttpRouter, JsonBody};
use nickel::status::StatusCode;
use rocksdb::{DB, Writable, IteratorMode, Direction, Options};
use rustc_serialize::json::{encode, Json, ToJson};


#[derive(RustcDecodable, RustcEncodable)]
struct TransactionLog {
    id: u64,
    facts: Vec<Fact>
}

#[derive(RustcDecodable, RustcEncodable)]
struct TransactionLedger {
    transaction: u64,
    facts: Vec<Fact>
}


#[derive(RustcDecodable, RustcEncodable)]
struct Fact {
    entity: u64,
    attribute: String,
    value: String,
    transaction: u64
}

#[derive(RustcDecodable, RustcEncodable)]
struct EntityLog {
    id: u64,
    time: u64
}

struct FactStorage {
    db: DB
}

impl FactStorage {
    pub fn new() -> FactStorage {
        FactStorage {
            db: DB::open_default("/tmp/rocksdb/facts").unwrap()
        }
    }

    pub fn store_fact(&self, fact: Fact) -> Result<(), &'static str> {
        let _ = self.db.put(
            fact.entity.to_string().as_bytes(),
            encode(&fact).unwrap().as_bytes()
        );
        Ok(())
    }
}

fn transaction_comparator(left: &[u8], right: &[u8]) -> i32 {
    let left_as_int = str::from_utf8(left).unwrap().parse::<u64>().unwrap();
    let right_as_int = str::from_utf8(right).unwrap().parse::<u64>().unwrap();
    if left_as_int < right_as_int {
        return -1;
    } else if left_as_int > right_as_int{
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    let mut server = Nickel::new();

    // server.post("/fact", middleware! { |request, mut response|
    //     match request.json_as::<Fact>() {
    //         Ok(fact) => {
    //             let storage = FactStorage::new();
    //             let _ = storage.store_fact(fact);
    //             ""
    //         }
    //         Err(e) => {
    //             response.set(StatusCode::BadRequest);
    //             ""
    //         }
    //     }
    // });
    // server.post("/entity", middleware! { |request, mut response|
    //     let db = DB::open_default("/tmp/rocksdb/transactions").unwrap();
    //     match request.json_as::<EntityLog>() {
    //         Ok(transaction_log) => {
    //             let _ = db.put(
    //                 transaction_log.id.to_string().as_bytes(),
    //                 transaction_log.time.to_string().as_bytes()
    //             );
    //             ""
    //         }
    //         Err(e) => {
    //             response.set(StatusCode::BadRequest);
    //             ""
    //         }
    //     }
    // });
    // server.get("/entity/latest", middleware! { |request, mut response|
    //     let db = DB::open_default("/tmp/rocksdb/transactions").unwrap();
    //     let mut iter = db.iterator(IteratorMode::End);  // Always iterates backward
    //     match iter.next() {
    //         Some((key, value)) => {
    //             let log = EntityLog {
    //                 id: str::from_utf8(&*key).unwrap().parse::<u64>().unwrap(),
    //                 time:  str::from_utf8(&*value).unwrap().parse::<u64>().unwrap()
    //             };
    //             encode(&log).unwrap()
    //         }
    //         None => String::from("")
    //     }
    // });
    server.post("/transaction", middleware! { |request, mut response|
        let mut opts = Options::default();
        opts.add_comparator("Transaction log comparator", transaction_comparator);
        opts.create_if_missing(true);
        let db = DB::open(&opts, "/tmp/rocksdb/transactions").unwrap();
        match request.json_as::<TransactionLog>() {
            Ok(transaction_log) => {
                let _ = db.put(
                    transaction_log.id.to_string().as_bytes(),
                    encode(&transaction_log).unwrap().as_bytes()
                );
                ""
            }
            Err(e) => {
                response.set(StatusCode::BadRequest);
                ""
            }
        }
    });
    server.get("/transaction/latest", middleware! { |request, mut response|
        let mut opts = Options::default();
        opts.add_comparator("Transaction log comparator", transaction_comparator);
        opts.create_if_missing(true);
        let db = DB::open(&opts, "/tmp/rocksdb/transactions").unwrap();
        let mut iter = db.iterator(IteratorMode::End);  // Always iterates backward
        match iter.next() {
            Some((key, value)) => {
                println!("key is {} val is {}", str::from_utf8(&*key).unwrap(), str::from_utf8(&*value).unwrap());
                str::from_utf8(&*value).unwrap().to_string()
            }
            None => String::from("")
        }
    });

    server.listen("127.0.0.1:3000");
}
