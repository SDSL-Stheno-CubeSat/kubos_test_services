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
    Logger::init("example-service").unwrap();

    Service::new(
        Config::new("example-service")
            .map_err(|err| {
                error!("Failed to load service config {:?}", err);
                err
            })
            .unwrap(),
        Subsystem::new(),
        QueryRoot,
        MutationRoot,
    )
    .start();

}

// cargo run --bin /home/dylan/Documents/rustprojects/kubos_projects/kubos_test_services/example_service/target/debug -- -c local_config.toml &
// cargo run -- -c ~/DevTools/kubos/tools/local_config.toml &
