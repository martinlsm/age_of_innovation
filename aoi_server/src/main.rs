use aoi_backend::gamemap::open_map;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    let _map = open_map();

    "Opened map!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
