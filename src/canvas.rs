use std::thread;
use std::time;
use std::mem;

extern crate rand;
extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
pub struct MyCanvas {
	sdl_context : sdl2::Sdl,
	sdl_canvas: sdl2::render::Canvas<Window>,

	width: u32,
	height: u32,

	rgb_buffer: Vec<u8>,
	z_buffer: Vec<f32>,

}

impl MyCanvas {

	#[allow(dead_code)]
	pub fn new(width: u32, height: u32) -> MyCanvas {
		let sdl_context = sdl2::init().unwrap();
		let sdl_video = sdl_context.video().unwrap();

		let sdl_window = sdl_video.window("rust-sdl2 demo: Video", width, height)
		    .position_centered()
		    .opengl()
		    .build()
		    .unwrap();

		let sdl_canvas = sdl_window.into_canvas().build().unwrap();

		MyCanvas {
			sdl_context: sdl_context,
			sdl_canvas: sdl_canvas,
			width: width,
			height: height,
			rgb_buffer: vec![0; (width * height * 3) as usize],
			z_buffer: vec![100000.0; (width * height) as usize],
		}
	}

	#[allow(dead_code)]
	pub fn clear(&mut self) {
		self.sdl_canvas.clear();
	}

	#[allow(dead_code)]
	pub fn set_depth (&mut self, x: i32, y: i32, d : f32){
		if x < 0 ||
		   y < 0 ||
		   x >= (self.width as i32) ||
		   y >= (self.height as i32)
		{
			return;
		}

		let i = (x as u32 + y as u32 * self.width) as usize;
		self.z_buffer[i] = d
	}
	#[allow(dead_code)]
	pub fn point (&mut self, x: i32, y: i32, color: u32){
		if x < 0 ||
		   y < 0 ||
		   x >= (self.width as i32) ||
		   y >= (self.height as i32)
		{
			return;
		}

		let i = 3 * (x as u32 + y as u32 * self.width) as usize;
		self.rgb_buffer[i + 0] = (color >> (8*2)) as u8;
		self.rgb_buffer[i + 1] = (color >> (8*1)) as u8;
		self.rgb_buffer[i + 2] = (color >> (8*0)) as u8;
	}
	#[allow(dead_code)]
	pub fn to_pix_coord (&self, x : f32, y : f32) ->(i32,  i32){
		(
			((x + 1.0) * (self.width  as f32)/ 2.0) as i32,
			((1.0 - y) * (self.height as f32)/ 2.0) as i32
		)
	}

	#[allow(dead_code)]
	pub fn present(&mut self){
		let texture_creator = self.sdl_canvas.texture_creator();

		let mut texture = texture_creator.create_texture_streaming(
		PixelFormatEnum::RGB24, self.width, self.height).unwrap();

		texture.with_lock(None, |buffer: &mut [u8], _: usize| {
			let n = (self.width * self.height * 3) as usize;
			for i in 0..n {
				buffer[i] = self.rgb_buffer[i];
			}
		}).unwrap();

		self.sdl_canvas.copy(&texture, None, None).unwrap();

		self.sdl_canvas.present();
	}

	#[allow(dead_code)]
	pub fn poll_events (&mut self) {
		let mut event_pump = self.sdl_context.event_pump().unwrap();
		for event in event_pump.poll_iter() {
			use sdl2::event::Event;

			match event {
				Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					//running = false
				},
				_ => {}
			}
		}
	}

	#[allow(dead_code)]
	pub fn wait_end (&mut self) {
/*
		let texture_creator = self.sdl_canvas.texture_creator();

		let mut texture = texture_creator.create_texture_streaming(
		PixelFormatEnum::RGB24, self.width, self.height).unwrap();

		texture.with_lock(None, |buffer: &mut [u8], _: usize| {
			let n = (self.width * self.height * 3) as usize;
			for i in 0..n {
				buffer[i] = self.rgb_buffer[i];
			}
		}).unwrap();

		self.sdl_canvas.copy(&texture, None, None).unwrap();

		self.sdl_canvas.present();
*/

		let mut running = true;
		let mut event_pump = self.sdl_context.event_pump().unwrap();
		while running {
			for event in event_pump.poll_iter() {
				use sdl2::event::Event;

				match event {
					Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						running = false
					},
					_ => {}
				}
			}
			thread::sleep(time::Duration::from_millis(1))
		}
	}
}
