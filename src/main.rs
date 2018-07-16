extern crate chrono;
extern crate gotham;
extern crate hyper;
extern crate mime;

#[macro_use]
extern crate gotham_derive;

#[macro_use]
extern crate serde_derive;

mod models;

use gotham::helpers::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::{Response, StatusCode};


#[derive(Deserialize, StateData, StaticResponseExtender)]
struct PathExtractor {
    email: String,
}

fn get_contact_handler(state: State) -> (State, Response) {
    let res = {
        let contact = PathExtractor::borrow_from(&state);

        create_response(
            &state,
            StatusCode::Ok,
            Some((
                format!("Email: {}", contact.email).into_bytes(),
                    mime::TEXT_PLAIN,
            )),
        )
    };

    (state, res)
}


fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/contacts/:email")
            .with_path_extractor::<PathExtractor>()
            .to(get_contact_handler);
    })
}

fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
