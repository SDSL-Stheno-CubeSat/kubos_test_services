use log::info
use std::io::{Error, ErrorKind};

// model for service's subsystem
#[derive(Clone)]
pub struct Subsystem {
    Arc<Mutex<RefCell<serial::SystemPort>>>,
}