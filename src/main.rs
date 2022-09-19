#[macro_use] extern crate rocket;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello!"
}

#[get("/world")]
fn world() -> &'static str {
    "world"
}

#[get("/<auction_id>")]
fn get_auction(auction_id: u64) -> String {
    return String::from("Auction #123");
    // let mut auc = Auction::new("Auction", "Auction Description", 100.00);
    // return auc.info();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, world])
        .mount("/auction", routes![get_auction])
}
