
use crate::color::*;

pub trait Texture { 
	fn value( &self, u : f32, v : f32 ) -> Color;
}

pub struct ConstantTexture {
	pub color : Color
}

impl Texture for ConstantTexture {
	fn value( &self, u : f32, v : f32 ) -> Color{
		self.color.clone()
	}
}


pub struct ChessTexture {
	pub color_a : Color,
	pub color_b : Color,
	pub scale : f32
}

use std::f32;

impl Texture for ChessTexture {
	fn value( &self, u : f32, v : f32 ) -> Color{
		let u = self.scale * u;
		let u = u - u.round();

		let v = self.scale * v;
		let v = v - v.round();

		let f = u * v > 0.0;
		let color = if f { self.color_a.clone()}else{self.color_b.clone()};
		return color;
	}
}
