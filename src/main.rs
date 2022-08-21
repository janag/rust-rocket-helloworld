#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World - Jana!😀"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
