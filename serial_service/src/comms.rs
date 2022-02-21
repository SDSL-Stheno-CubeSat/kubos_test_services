// Return type for this service.
type ServiceResult<T> = Result<T, Error>;

//use comms_service::*;
use failure::*;
//use kubos_service::Logger;
use log::*;

use serial;
use serial::prelude::*;
use std::cell::RefCell;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

const BUS: &str = "/dev/ttyS1";
//const BUS: &str = "/dev/ttyACM0";

const TIMEOUT: Duration = Duration::from_millis(1000);

// Initialize the serial bus connection for reading and writing from/to the "radio"
pub fn serial_init(bus: &str) -> ServiceResult<Arc<Mutex<RefCell<serial::SystemPort>>>> {
    let settings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };

    let mut port = serial::open(bus)?;

    port.configure(&settings)?;
    port.set_timeout(TIMEOUT)?;

    // Wrap the port in a mutex so that multiple threads can access it
    let conn = Arc::new(Mutex::new(RefCell::new(port)));

    Ok(conn)
}



// The read function that the comms service read thread will call to wait for messages from the
// "radio"
// Returns once a message has been received
// MAX_READ specific to beagleboard, check kubos docs for regular max read size
const MAX_READ: usize = 48;
pub fn read(conn: &Arc<Mutex<RefCell<serial::SystemPort>>>) -> ServiceResult<Vec<u8>> {
    loop {
        // Note: These brackets force the program to release the serial port's mutex so that any
        // threads waiting on it in order to perform a write may do so
        {
            // Take ownership of the serial port
            let conn = match conn.lock() {
                Ok(val) => val,
                Err(e) => {
                    error!("Failed to take mutex: {:?}", e);
                    panic!();
                }
            };
            let mut conn = conn.try_borrow_mut()?;

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

        // Sleep for a moment so that other threads have the chance to grab the serial port mutex
        //thread::sleep(Duration::from_millis(10));
    }
}



// The write function that the comms service will use to write messages to the "radio"
//
// This function may be called from either a message handler thread or from a downlink endpoint
// ex:
//
// let msg = String::from("Hello back");
// let enmsg = msg.as_bytes();
// let _wr = write(&conn, &enmsg);
//
pub fn write(conn: &Arc<Mutex<RefCell<serial::SystemPort>>>, msg: &[u8]) -> ServiceResult<()> {
    let conn = match conn.lock() {
        Ok(val) => val,
        Err(e) => bail!("Failed to take mutex: {:?}", e),
    };
    let mut conn = conn.try_borrow_mut()?;

    conn.write(msg).and_then(|num| {
        debug!("Wrote {} bytes to radio", num);
        Ok(())
    })?;

    Ok(())
}


fn main() -> ServiceResult<()> {

    // initialize serial port
    let conn = serial_init(BUS)?;

    thread::sleep(Duration::from_millis(3000));

    // write a message
    let msg = String::from("ping - beagleboard");
    let enmsg = msg.as_bytes();
    let _wr = write(&conn, &enmsg);

    // keep reading and printing messages if they are received
    loop {
        let bstr = read(&conn)?;
        let s = String::from_utf8_lossy(&bstr);
        if s.chars().count() > 0 {
            println!("{}", s);
    }
}
