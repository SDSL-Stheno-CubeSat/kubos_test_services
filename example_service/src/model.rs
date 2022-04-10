use log::info;
use std::io::{Error, ErrorKind};

/// Model for power mutations
pub struct SetPower {
    pub power: bool,
}

/// Model for uptime mutations
pub struct ResetUptime {
    pub uptime: i32,
}

/// Model for thermometer mutations
pub struct CalibrateThermometer {
    pub temperature: i32,
}


/// Model for service's subsystem
#[derive(Clone)]
pub struct Subsystem;

impl Subsystem {
    /// Creates new Subsystem structure instance
    /// Code initializing subsystems communications
    /// would likely be placed here
    pub fn new() -> Subsystem {
        info!("Getting new subsystem data");
        Subsystem {}
    }

    /// Power status getter
    /// Code querying for new power value
    /// could be placed here
    pub fn power(&self) -> Result<bool, Error> {
        info!("Getting power");
        // Low level query here
        Ok(true)
    }

    /// Power state setter
    /// Here we would call into the low level
    /// device function
    pub fn set_power(&self, _power: bool) -> Result<SetPower, Error> {
        info!("Setting power state");
        // Send command to device here
        if _power {
            Ok(SetPower { power: true })
        } else {
            Err(Error::new(
                ErrorKind::PermissionDenied,
                "Cannot power off device",
            ))
        }
    }

    /// Uptime getter
    /// Code querying for new uptime value
    /// could be placed here
    pub fn uptime(&self) -> Result<i32, Error> {
        info!("Getting uptime");
        // Low level query here
        Ok(111_001)
    }

    /// Uptime reset function
    pub fn reset_uptime(&self) -> Result<ResetUptime, Error> {
        info!("Resetting uptime");
        // Send command to device here
        Ok(ResetUptime { uptime: 0 })
    }

    /// Temperature getter
    /// Demonstrates returning an error condition
    pub fn temperature(&self) -> Result<i32, Error> {
        info!("Getting temperature");
        // Low level query here
        Err(Error::new(
            ErrorKind::TimedOut,
            "Failed to retrieve temperature",
        ))
    }

    /// Temperature calibration
    /// Demonstrates a mutation with error condition
    pub fn calibrate_thermometer(&self) -> Result<CalibrateThermometer, Error> {
        info!("Calibrating thermometer");
        Ok(CalibrateThermometer { temperature: 98 })
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