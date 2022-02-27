use failure::{Error, SyncFailure};
use kubos_service::Logger;
use log::{debug,error};

fn main() -> Result<(), Error> {
    println!("Start logging");
    Logger::init("log-test").unwrap();

    debug!("this is a debug {}", "message");
    error!("this is an error!");
    Ok(())
}
