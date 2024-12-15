use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution};
use nokhwa::Camera;

fn print_buffer_hex(buffer: &[u8]) {
    for (i, byte) in buffer.iter().enumerate() {
        if i % 16 == 0 && i > 0 {
            println!();
        }
        print!("{:02X} ", byte);
    }
    println!(); // Newline at the end
}
fn main() {
    let index = CameraIndex::Index(0);

    // Request the highest resolution CameraFormat that can be decoded to RGB
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(CameraFormat::new(Resolution::new(640, 480), FrameFormat::MJPEG, 30)));

    // Try to initialize the camera
    let camera = Camera::new(index, requested);

    match camera {
        Ok(mut camera) => {
            // Try to capture a frame
            camera.open_stream().unwrap();
            let frame = camera.frame();

            match frame {
                Ok(frame) => {
                    println!("Captured Single Frame of {}", frame.buffer().len());
                    print_buffer_hex(frame.buffer());

                    // Try to decode the frame into an ImageBuffer
                    // let decoded = frame.decode_image::<RgbFormat>()
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
