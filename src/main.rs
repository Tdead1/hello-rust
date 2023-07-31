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
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let file = BufReader::new(File::open("data/guitar.mp3").unwrap());
	let decoder = Decoder::new(file).unwrap();
	let audio_result: Result<(), rodio::PlayError> = stream_handle.play_raw(decoder.convert_samples());
    
	while window.render() && audio_result.is_ok()
	{
        cube.prepend_to_local_rotation(&rot);
    }
}