use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

fn main() {
    let index = CameraIndex::Index(1);

    // Request the highest resolution CameraFormat that can be decoded to RGB
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // Try to initialize the camera
    let camera = Camera::new(index, requested);

    match camera {
        Ok(mut camera) => {
            // Try to capture a frame
            let frame = camera.frame();

            match frame {
                Ok(frame) => {
                    println!("Captured Single Frame of {}", frame.buffer().len());

                    // Try to decode the frame into an ImageBuffer
                    let decoded = frame.decode_image::<RgbFormat>();

                    match decoded {
                        Ok(decoded_image) => {
                            println!("Decoded Frame of {}", decoded_image.len());
                        }
                        Err(e) => {
                            eprintln!("Error decoding frame: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error capturing frame: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error initializing camera: {}", e);
        }
    }
}
