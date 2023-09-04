use aoi_backend::map;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    let _map = map::open_map(map::MapId::Base);

    "Opened map!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
