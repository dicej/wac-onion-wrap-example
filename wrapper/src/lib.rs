wit_bindgen::generate!({
    path: "../wit",
    world: "wrapper",
    exports: {
        "onion:onion/handler": Component,
    }
});

use {
    exports::onion::onion::handler::{Guest, Request, Response},
    onion::onion::handler,
};

struct Component;

impl Guest for Component {
    fn handle(request: Request) -> Response {
        Response::new(&format!(
            "(wrapper.handle {})",
            handler::handle(request).body()
        ))
    }
}
