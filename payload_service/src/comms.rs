//!
//! Serial communications functionality for use in conjunction
//! with the communications service library. KISS framing is
//! implemented for data integrity over the serial link.
//!

use crate::SerialServiceResult;
use rust_uart::Connection;
use std::time::Duration;

//use failure::*;

pub struct SerialComms {
    conn: Connection,
}

impl SerialComms {
    pub fn new(path: &str) -> Self {
        let serial_settings = serial::PortSettings {
            baud_rate: serial::Baud115200,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };

        let conn = Connection::from_path(path, serial_settings, Duration::from_millis(1)).unwrap();

        SerialComms {
            conn,
        }
    }

    // Function to allow reading a whole UDP packet from a serial socket
    pub fn read(&self) -> SerialServiceResult<Vec<u8>> {
        //let mut buffer = self.buffer.borrow_mut();
        let mut buffer: Vec<u8> = Vec::new();
        while let Ok(mut buf) = self.conn.read(1, Duration::from_millis(1)) {
            buffer.append(&mut buf);
            if buffer.len() > 4096 {
                break;
            }
        }

        let message = buffer.to_vec();

        Ok(message)
    }

    // Function to allow writing over a UDP socket.
    pub fn write(&self, data: &[u8]) -> SerialServiceResult<()> {
        match self.conn.write(&data) {
            Ok(result) => Ok(result),
            Err(_err) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unable to write to UART")),
        }
    }
}