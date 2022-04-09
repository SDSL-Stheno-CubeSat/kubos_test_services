#![deny(warnings)]
#![allow(deprecated)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;
mod comms;

extern crate rust_uart;
use crate::model::Subsystem;
use crate::schema::{MutationRoot, QueryRoot};
use kubos_service::{Config, Logger, Service};
use log::error;
use std::io::Error;
use crate::comms::*;
use std::sync::{Arc, Mutex};

type SerialServiceResult<T> = Result<T, Error>;

fn main() {

    let bus = "/dev/ttyS1";

    let conn = Arc::new(Mutex::new(SerialComms::new(&bus)));

    Logger::init("payload-service").unwrap();

    Service::new(
        Config::new("payload-service")
            .map_err(|err| {
                error!("Failed to load service config {:?}", err);
                err
            })
            .unwrap(),
        Subsystem::new(conn),
        QueryRoot,
        MutationRoot,
    )
    .start();

}
