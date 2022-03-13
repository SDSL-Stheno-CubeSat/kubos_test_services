use log::info;
use std::io::{Error, ErrorKind};
use failure::*;
use log::*;
use rust_uart::*;
use std::sync::{Arc, Mutex};
use serial;

pub struct TxSuccess {
    pub success: bool,
}

// model for service's subsystem
#[derive(Clone)]
pub struct Subsystem {
    device: Arc<Mutex<Connection>>,
}

impl Subsystem {
    pub fn new(device: Arc<Mutex<Connection>>) -> Subsystem {
        info!("Getting new subsystem");

        Subsystem { device }
    }

    pub fn uart_tx(&self, data: String) -> Result<TxSuccess, Error> {
        info!("Transmitting data");

        if let Ok(device) = self.device.lock() {
            match device.write(data.as_bytes()) {
                Ok(_result) => Ok(TxSuccess { success: true }),
                Err(_err) => Ok(TxSuccess { success: false }),
            }
        } else {
            Err(Error::new(
                ErrorKind::PermissionDenied, "Failed to aquire mutex lock",
            ))
        }
    }

    pub fn uart_rx(&self) -> Result<Vec<u8>, Error> {
        const MAX_READ: usize = 48;
        loop {
            // Note: These brackets force the program to release the serial port's mutex so that any
            // threads waiting on it in order to perform a write may do so
            {
                // Take ownership of the serial port
                let conn = match self.device.lock() {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Failed to take mutex: {:?}", e);
                        panic!();
                    }
                };
    
                // Loop until either a full message has been received or a non-timeout error has occured
                let mut packet = vec![];
                let mut totalbytes = 0;
                loop {
                    let mut buffer: Vec<u8> = vec![0; MAX_READ];
                    match conn.read(buffer.as_mut_slice()) {
                        Ok(num) => {
                            buffer.resize(num, 0);
                            packet.append(&mut buffer);
    
                            totalbytes += num;
    
                            if num == 0 || totalbytes >= MAX_READ {
                                return Ok(packet);
                            }
                        }
                        Err(ref err) => match err.kind() {
                            ::std::io::ErrorKind::TimedOut => {
                                return Ok(packet);
                            }
                            other => bail!("Radio read failed: {:?}", other),
                        },
                    };
                }
            }
        }
    }
}
