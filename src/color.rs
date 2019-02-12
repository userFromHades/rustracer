use std::ops::{Add, Sub, Mul, Div};
use std::f32;

#[derive(Debug, Clone, Copy)]
pub struct Color {
	pub r : f32,
	pub g : f32,
	pub b : f32,
}

fn clamp(input:f32, min: f32, max: f32) -> f32 {
	assert!(max >= min);
	let mut x = input;
	if x < min { x = min; }
	if x > max { x = max; }
	x
}

impl Color {
	pub fn new (r : f32, g :f32, b :f32) -> Color {
		Color{ r, g, b }
	}

	pub fn as_u32(&self) -> u32 {
		let r  = (clamp( self.r, 0.0, 1.0) * 255.0) as u32;
		let g  = (clamp( self.g, 0.0, 1.0) * 255.0) as u32;
		let b  = (clamp( self.b, 0.0, 1.0) * 255.0) as u32;

		0xff000000 | (r << 16) | (g <<8) | (b << 0)
	} 
}

impl Add for Color {
	type Output = Color;

	fn add(self, other: Color) -> Color {
		Color { r: self.r + other.r, 
		        g: self.g + other.g,
		        b: self.b + other.b }
	}
}

impl Add<&Color> for Color {
	type Output = Color;

	fn add(self, other: &Color) -> Color {
		Color { r: self.r + other.r, 
		        g: self.g + other.g, 
		        b: self.b + other.b }
	}
}

impl Add<Color> for &Color {
	type Output = Color;

	fn add(self, other: Color) -> Color {
		Color { r: self.r + other.r, 
		        g: self.g + other.g, 
		        b: self.b + other.b }
	}
}

impl<'a> Add for &'a Color {
	type Output = Color;

	fn add(self, other: &'a Color) -> Color {
		Color{ r: self.r + other.r, 
		       g: self.g + other.g,
		       b: self.b + other.b}
	}
}

impl Sub<Color> for Color {
	type Output = Color;

	fn sub(self, other: Color) -> Color {
		Color { r: self.r - other.r, 
		        g: self.g - other.g, 
		        b: self.b - other.b }
	}
}

impl Sub<Color> for &Color {
	type Output = Color;

	fn sub(self, other: Color) -> Color {
		Color { r: self.r - other.r, 
		        g: self.g - other.g, 
		        b: self.b - other.b }
	}
}

impl Sub<&Color> for Color {
	type Output = Color;

	fn sub(self, other: &Color) -> Color {
		Color { r: self.r - other.r, 
		        g: self.g - other.g, 
		        b: self.b - other.b }
	}
}

impl Sub<&Color> for &Color {
	type Output = Color;

	fn sub(self, other: &Color) -> Color {
		Color { r: self.r - other.r, 
		        g: self.g - other.g, 
		        b: self.b - other.b }
	}
}

impl Mul<f32> for Color {
	type Output = Color;

	fn mul(self, other: f32) -> Color {
		Color { r: self.r * other, 
		        g: self.g * other, 
		        b: self.b * other }
	}
}

impl Mul<f32> for &Color {
	type Output = Color;

	fn mul(self, other: f32) -> Color {
		Color { r: self.r * other, 
		        g: self.g * other, 
		        b: self.b * other }
	}
}

impl Mul<Color> for f32 {
	type Output = Color;

	fn mul(self, other: Color) -> Color {
		Color { r: self * other.r, 
		        g: self * other.g, 
		        b: self * other.b }
	}
}

impl Mul<&Color> for f32 {
	type Output = Color;

	fn mul(self, other: &Color) -> Color {
		Color { r: self * other.r, 
		        g: self * other.g, 
		        b: self * other.b }
	}
}


impl Div<f32> for Color {
	type Output = Color;

	fn div(self, other: f32) -> Color {
		Color { r: self.r / other, 
		        g: self.g / other, 
		        b: self.b / other }
	}
}

impl<'a> Div<f32> for &'a Color {
	type Output = Color;

	fn div(self, other: f32) -> Color {
		Color { r: self.r / other, 
		        g: self.g / other, 
		        b: self.b / other }
	}
}
