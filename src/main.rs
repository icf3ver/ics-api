#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;


mod parse_ics;

//hello
#[get("/api/<month>/<day>/<year>")]
fn api_class(month: u8, day: u8, year: u16) -> String{
    let a: String = parse_ics::date_event(month, day, year);
    a
}

//main
fn main() {
    parse_ics::download();
    parse_ics::parse();
    rocket::ignite().mount("/", routes![api_class]).launch();
}