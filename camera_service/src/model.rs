use log::info;
use std::io::{Error, ErrorKind};

// Model for image
pub struct Image {
    pub bytes: Vec<i32>,
    pub resolution: Resolution,
}

// Model for camera mutations
pub struct Camera {
    pub started: bool,
    pub resolution: Resolution,
}

impl Camera {
    fn default() -> Camera {
        Camera {
            started: true,
            resolution: Resolution::default(),
        }
    }
}

// Model for resolution mutations
pub struct Resolution {
    pub x: u32,
    pub y: u32,
}

impl Resolution {
    fn default() -> Resolution {
        Resolution {
            x: 1920,
            y: 1080,
        }
    }
}

/// Model for service's subsystem
#[derive(Clone)]
pub struct Subsystem;

impl Subsystem {
    
    // Subsystem constructor
    pub fn new() -> Subsystem {
        info!("Creating a new Camera Subsystem");
        Subsystem {}
    }

    // Query methods

    // Capture query method
    // TODO: Should return a byte stream where the image is stored
    pub fn capture(&self) -> Result<Image, Error> {
        info!("Capturing image");
        // Low level image capturing logic here(Should return an Image object with 
        // stream of bytes as a vector)
        let image: Result<Image, Error> = Ok(Image {
            bytes: vec![1, 1, 1, 1],
            resolution: Resolution { x: 1920, y: 1080}
        }); // example representation of an Image object

        match image {
            Ok(image) => Ok(image),
            _ => Err(Error::new(
                ErrorKind::Other,
                "Error occured while capturing image",
            ))
        }
    }

    // Mutation methods
    // TODO: Optimize memory by returning what is required from the mutations

    // Method to start the camera
    pub fn start_camera(&self) -> Result<bool, Error> {
        info!("Starting the camera...");
        // Low level logic to start the camera
        let camera: Camera = Camera::default(); // low level function goes here

        if camera.started {
            Ok(true)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Error occured while starting the camera",
            ))
        }
    }

    // Method to start the camera
    pub fn stop_camera(&self) -> Result<bool, Error> {
        info!("Stopping the camera...");
        // Low level logic to stop the camera
        let stopped: bool = true; // low level function goes here
        
        if stopped {
            Ok(true)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Error occured while stopping the camera",
            ))
        }
    }


    // Method for setting camera resolution
    pub fn set_resolution(&self, x: i32, y: i32) -> Result<bool, Error> {
        info!("Setting resolution");
        let resolution = Resolution {
                x: x as u32,
                y: y as u32,
            };
        
        // low level logic to set resolution
        let is_set: bool = resolution.x == resolution.x;
        // TODO: Might involve more checks like whether camera is on
        if is_set {
            Ok(true)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Error occured while setting the resolution",
            ))
        }
    }

}

// Overriding the destructor
impl Drop for Subsystem {
    // Clean up subsystem
    fn drop(&mut self) {
        info!("Destructing subsystem");
    }
}