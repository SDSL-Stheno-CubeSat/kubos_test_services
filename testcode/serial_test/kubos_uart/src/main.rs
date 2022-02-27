// FOR RADIO - https://m17-protocol-specification.readthedocs.io/en/latest/kiss_protocol.html
// I belive you must send start and end bits over the radio communication to use th KISS framing

mod comms;

use crate::comms::*;

extern crate rust_uart;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
extern crate serial;
extern crate log;
//use kubos_service::Logger;

use failure::*;

type SerialServiceResult<T> = Result<T, Error>;

fn main() {
    //Logger::init("serial-comms-service").unwrap();

    let bus = "/dev/ttyS1";

    let serial_comms = Arc::new(Mutex::new(SerialComms::new(&bus)));

    let mut timer = SystemTime::now();

    loop {

        let mut raw_data: Vec<u8> = Vec::new();
        match read_ser(&serial_comms) {
            Ok(st) => {
                raw_data = st;
            }
            Err(_e) => {
                println!("Error occured: {:?}", _e);
                // No data found!
            }
        }
        let s = String::from_utf8_lossy(&raw_data);
        if s.chars().count() > 0 {
            println!("{}", s);
        }

        match timer.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs() >= 5 {
                    let msg = String::from("led");
                    let enmsg = msg.as_bytes();
                    let _res = write_ser(&serial_comms, &enmsg);
                    
                    timer = SystemTime::now();
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}