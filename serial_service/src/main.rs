#![deny(warnings)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;

use crate::model::Subsystem;
use crate::schema::{MutationRoot, QueryRoot};
use kubos_service::{Config, Logger, Service};
use log::error;

fn main() {
    Logger::init("serial-service").unwrap();

    Service::new(
        Config::new("serial-service")
            .map_err(|err| {
                error!("Failed to load service config: {:?}", err);
                err
            })
            .unwrap(),
        Subsystem::new(),
        QueryRoot,
        MutationRoot,
    )
    .start();
}
