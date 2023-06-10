extern crate kiss3d;

use kiss3d::nalgebra::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut cube = window.add_cube(0.5, 0.5, 0.5);

    cube.set_color(1.0, 0.0, 0.0);
    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

	// Get a output stream handle to the default physical sound device
	// Load a sound from a file, using a path relative to Cargo.toml
	// Decode that sound file into a source
	// Play the sound directly on the device
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let file = BufReader::new(File::open("data/guitar.mp3").unwrap());
	let source = Decoder::new(file).unwrap();
	let err: Result<(), rodio::PlayError> = stream_handle.play_raw(source.convert_samples());

	if !err.is_ok()
	{
   		cube.set_color(0.0, 1.0, 0.0);
	}

    while window.render() 
	{
        cube.prepend_to_local_rotation(&rot);
    }
}