wit_bindgen::generate!({
    path: "../wit",
    world: "wrapper",
    exports: {
        "onion:onion/handler": Component,
    }
});

use exports::onion::onion::handler::{Guest, Request, Response};

struct Component;

impl Guest for Component {
    fn handle(request: Request) -> Response {
        Response::new(&format!("(service.handle {})", request.body()))
    }
}
