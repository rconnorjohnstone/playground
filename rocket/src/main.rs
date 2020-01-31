#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate ical;

use std::io::BufReader;
use std::fs::File;

#[get("/")]
fn index() -> String {
    let buf = BufReader::new(File::open("static/example.ics").unwrap());
    let reader = ical::IcalParser::new(buf);
    let mut sample = String::from("");
    for line in reader {
        sample = format!("{}{:?}", sample, line.properties);
    }
    sample
}

#[get("/<name>")]
fn hello(name: String) -> String {
    format!("Hello {}", name)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello]).launch();
}
