use rocket::{launch, routes};
use shuffle_server::{get_subtrees_endpoint, move_item_endpoint};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_subtrees_endpoint, move_item_endpoint])
}
