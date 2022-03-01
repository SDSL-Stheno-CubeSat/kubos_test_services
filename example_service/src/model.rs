use log::info;
use std::io::{Error, ErrorKind};

// model for power mutations
pub struct SetPower {
    pub power: bool,
}

// model for uptime mutations
pub struct ResetUptime {
    pub uptime: i32,
}

// thermometer mutations
pub struct CailbrateThermometer {
    pub temperature: i32,
}

// model for service subsystem
#[derive(Clone)]
pub struct Subsystem;

impl Subsystem {
    pub fn new() -> Subsystem {
        // Create new subsystem
        // initializing comms would be here
        info!("Getting new subsystem data");
        Subsystem {}
    }

    // power status getter
    // If powered = true, if not = false
    pub fn power(&self) -> <Resultbool, Error> {
        info("Getting power");
        Ok(true);
    }

    // power state setter
    // I believe Result<> requires a struct, that is why the bool is wrapped in the structure
    pub fn set_power(&self, _power: bool) -> Result<SetPower, Error> {
        info!("Setting power state");

        // send command to device

        if _power {
            // return that this was successful
            Ok(SetPower { power: true })
        } else {
            Err(Error::new(
                ErrorKind::PermissionDenied, "No can do, cannot power off",
            ))
        }
    }

    // Uptime getter
    pub fn uptime(&self) -> Result<i32, Error> {
        info!("Getting uptime");
        Ok(111_001)
    }

    // temperature getter
    // demonstrate returning an error
    pub fn temerature(&self) -> Result<i32, Error> {
        info!("Getitng temperature");
        Err(Error::new(
            ErrorKind::TimedOut,
            "Failed to retrieve temperature",
        ))
    }

    // temp calibration
    // demonstrate mutation with an error
    pub fn calibrate_thermometer(&self) -> Result<CalibrateThermometer, Error> {
        info!("Calibrating thermometer")
        Ok(CalibrateThermometer { temperature: 98 })
    }

    // override the destructure
    impl Drop for Subsystem {
        // clean up comms stuff
        fn drop(&mut self) {
            info!("Destructing subsystem");
        }
    }
}
