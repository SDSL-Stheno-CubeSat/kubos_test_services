use log::info;
use std::io::{Error, ErrorKind};
//use failure::*;
//use log::*; TODO
use std::sync::{Arc, Mutex};
use crate::comms::*;

pub struct TxSuccess {
    pub success: bool,
}

pub struct RxReading {
    pub success: bool,
    pub data: String,
}

// model for service's subsystem
#[derive(Clone)]
pub struct Subsystem {
    device: Arc<Mutex<SerialComms>>,
}

impl Subsystem {
    pub fn new(device: Arc<Mutex<SerialComms>>) -> Subsystem {
        info!("Getting new subsystem");

        Subsystem { device }
    }

    pub fn uart_tx(&self, data: String) -> Result<TxSuccess, Error> {
        info!("Transmitting data");

        if let Ok(device) = self.device.lock() {
            match device.write(data.as_bytes()) {
                Ok(_result) => Ok(TxSuccess { success: true }),
                Err(_err) => Ok(TxSuccess { success: false })
            }
        } 
        else {
            Err(Error::new(ErrorKind::PermissionDenied, "Failed to acquire mutex lock"))
        }
    }

    pub fn uart_rx(&self) -> Result<RxReading, Error> {
        info!("Receiving data!");

        if let Ok(device) = self.device.lock() {
            match device.read() {
                Ok(_result) => Ok(RxReading { success: true, data: String::from_utf8_lossy(&_result).to_string() }),
                Err(_err) => Ok(RxReading { success: false, data: "".to_string() }),
            }
        } 
        else {
            Err(Error::new(ErrorKind::PermissionDenied, "Failed to acquire mutex lock"))
        }
    }
}

/// Overriding the destructor
impl Drop for Subsystem {
    /// Here is where we would clean up
    /// any subsystem communications stuff
    fn drop(&mut self) {
        info!("Destructing subsystem");
    }
}