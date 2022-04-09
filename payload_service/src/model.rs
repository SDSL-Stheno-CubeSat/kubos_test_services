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

pub struct SetPower {
    pub power: bool,
}

pub struct CalibrateThermometer {
    pub success: bool,
}


#[derive(Clone)]
pub struct Subsystem {
    device: Arc<Mutex<SerialComms>>,
}

impl Subsystem {
    pub fn new(device: Arc<Mutex<SerialComms>>) -> Subsystem {
        info!("Getting new subsystem");

        Subsystem { device }
    }

    // Used for transmitting data, and probably in place of "commandRaw"
    pub fn uart_tx(&self, data: String) -> Result<TxSuccess, Error> {
        info!("Transmitting data");

        if let Ok(locked_device) = self.device.lock() {
            match locked_device.write(data.as_bytes()) {
                Ok(_result) => Ok(TxSuccess { success: true }),
                Err(_err) => Ok(TxSuccess { success: false }),
            }
        }
        else {
            Err(Error::new(ErrorKind::PermissionDenied, "Failed to acquire mutex lock"))
        }
    }

    // Used for reading data
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

    // Base function for getters, sends command and recieves answer. NOT used in the schema
    // ONLY for the model
    fn getter(&self, command: String) -> Result<String, Error>
    {
        // Send command to request data from device
        let _t: TxSuccess = match self.uart_tx(command) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        // Read data from device for answer
        let _reading: RxReading = match self.uart_rx() {
            Ok(result) => return Ok(result.data),
            Err(err) => return Err(err),
        };
    }

    // power getter
    pub fn power(&self) -> Result<bool, Error> {
        info!("Getting power");

        let command = String::from("power");
        let result = Self::getter(&self, command)?;

        // arduino returns a 1 or 0 for true or false
        if result.trim().parse::<i32>().unwrap() == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // power setter
    pub fn set_power(&self, _power: bool) -> Result<SetPower, Error> {
        info!("Setting power state");
        // send command
        // this line creates the command "power:_power" where _power is true or false
        let command = ["power",&_power.to_string()].join(":");
        let _t: TxSuccess = match self.uart_tx(command) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        Ok(SetPower { power: _power })
    }

    // temperature getter
    pub fn temperature(&self) -> Result<i32, Error> {
        info!("Getting temperature");

        let command = String::from("temperature");
        let result = Self::getter(&self, command)?;

        println!("result: {}", result.trim());
        Ok(result.trim().parse::<i32>().unwrap())
    }

    // temperature setter
    pub fn calibrate_thermometer(&self) -> Result<CalibrateThermometer, Error> {
        info!("Calibrating thermometer");
        // send command
        let command = String::from("thermometer calibration");
        let _t: TxSuccess = match self.uart_tx(command) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        Ok(CalibrateThermometer { success: true })
    }
}