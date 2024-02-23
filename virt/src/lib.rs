wit_bindgen::generate!({
    path: "../wit",
    world: "virt",
    exports: {
        "onion:onion/types/request": Request,
        "onion:onion/types/response": Response,
    }
});

use {
    exports::onion::onion::types::{GuestRequest, GuestResponse},
    onion::onion::types::{Request as HostRequest, Response as HostResponse},
};

pub struct Request(HostRequest);

impl GuestRequest for Request {
    fn body(&self) -> String {
        format!("(virt-request.body {})", self.0.body())
    }
}

pub struct Response(HostResponse);

impl GuestResponse for Response {
    fn new(body: String) -> Self {
        Self(HostResponse::new(&format!("(virt-response.new {body})")))
    }

    fn body(&self) -> String {
        format!("(virt-response.body {})", self.0.body())
    }
}
