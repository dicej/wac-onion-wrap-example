wasmtime::component::bindgen!({
    path: "../wit",
    world: "service",
    async: true,
    with: {
        "onion:onion/types/request": Request,
        "onion:onion/types/response": Response,
    }
});

use {
    anyhow::Result,
    async_trait::async_trait,
    tokio::fs,
    wasmtime::{
        component::{Component, Linker, Resource, ResourceTable},
        Config, Engine, Store,
    },
};

struct Ctx {
    table: ResourceTable,
}

#[async_trait]
impl onion::onion::types::HostRequest for Ctx {
    async fn body(&mut self, this: Resource<Request>) -> Result<String> {
        Ok(format!(
            "(host-request.body {})",
            self.table.get(&this)?.body
        ))
    }

    fn drop(&mut self, this: Resource<Request>) -> Result<()> {
        self.table.delete(this)?;
        Ok(())
    }
}

#[async_trait]
impl onion::onion::types::HostResponse for Ctx {
    async fn new(&mut self, body: String) -> Result<Resource<Response>> {
        Ok(self.table.push(Response {
            body: format!("(host-response.new {body})"),
        })?)
    }

    async fn body(&mut self, this: Resource<Response>) -> Result<String> {
        Ok(format!(
            "(host-response.body {})",
            self.table.get(&this)?.body
        ))
    }

    fn drop(&mut self, this: Resource<Response>) -> Result<()> {
        self.table.delete(this)?;
        Ok(())
    }
}

impl onion::onion::types::Host for Ctx {}

pub struct Request {
    body: String,
}

pub struct Response {
    body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let component = Component::new(&engine, fs::read("composed.wasm").await?)?;

    let mut linker = Linker::new(&engine);

    Service::add_to_linker(&mut linker, |ctx| ctx)?;

    let mut store = Store::new(
        &engine,
        Ctx {
            table: ResourceTable::new(),
        },
    );

    let (service, _) = Service::instantiate_async(&mut store, &component, &linker).await?;

    let request = store
        .data_mut()
        .table
        .push(Request { body: "foo".into() })?;

    let response = service
        .onion_onion_handler()
        .call_handle(&mut store, request)
        .await?;

    let response = store.data_mut().table.delete(response)?;

    // TODO: update the string literal to match the expected value
    assert_eq!("<todo>", &response.body);

    Ok(())
}
