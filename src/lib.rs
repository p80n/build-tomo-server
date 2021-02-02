#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod handlers;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/",
                           routes![
                               handlers::build::build,
                               handlers::healthz::healthz
                           ]
    )
}
