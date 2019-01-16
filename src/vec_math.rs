use std::ops::{Add, Sub, Mul, Div, Neg};

pub struct Vec2{
	pub x : f32,
	pub y : f32
}

impl Vec2 {

	#[allow(dead_code)]
	pub fn new (x: f32, y: f32) -> Vec2 {
		Vec2 {x: x, y: y}
	}
}
//-----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Vec3{
	pub x : f32,
	pub y : f32,
	pub z : f32
}

impl Vec3 {

	#[allow(dead_code)]
	pub fn new (x: f32, y: f32, z: f32) -> Vec3 {
		Vec3 {x: x, y: y, z: z}
	}

	#[allow(dead_code)]
	pub fn ones () -> Vec3 {Vec3 {x: 1.0, y: 1.0, z: 1.0}}

	#[allow(dead_code)]
	pub fn zero () -> Vec3 {Vec3 {x: 0.0, y: 0.0, z: 0.0}}

	#[allow(dead_code)]
	pub fn length(&self) -> f32 {
		(self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
	}

	#[allow(dead_code)]
	pub fn squre_length(&self) -> f32 {
		self.x*self.x + self.y*self.y + self.z*self.z
	}

	#[allow(dead_code)]
	pub fn normalize(&mut self){
		let l = self.length();
		self.x /= l;
		self.y /= l; 
		self.z /= l;
	}

	#[allow(dead_code)]
	pub fn normalized(&self) -> Vec3{
		let l = self.length();
		Vec3 { x: self.x / l, y: self.y / l, z: self.z / l}
	}

	#[allow(dead_code)]
	pub fn scale(&mut self, s : f32){
		self.x *= s;
		self.y *= s;
		self.z *= s;
	}

	#[allow(dead_code)]
	pub fn scaled(&self, s : f32) -> Vec3{
		Vec3 { x: self.x * s, y: self.y * s, z: self.z * s}
	}
}

impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

impl Add<&Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, other: &Vec3) -> Vec3 {
		Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

impl Add<Vec3> for &Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

impl<'a> Add for &'a Vec3 {
	type Output = Vec3;

	fn add(self, other: &'a Vec3) -> Vec3 {
		Vec3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}

impl Sub<Vec3> for &Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}

impl Sub<&Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, other: &Vec3) -> Vec3 {
		Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}

impl Sub<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn sub(self, other: &Vec3) -> Vec3 {
		Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}



#[allow(dead_code)]
pub fn dot_product(v1 : &Vec3, v2 : &Vec3) -> f32{
	v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}


#[allow(dead_code)]
pub fn cross_product(v1 : &Vec3, v2 : &Vec3) -> (Vec3){
	Vec3 { x: v1.y * v2.z - v1.z * v2.y,
	       y: v1.z * v2.x - v1.x * v2.z,
	       z: v1.x * v2.y - v1.y * v2.x }
}

#[allow(dead_code)]
pub fn reflect(v : &Vec3, n : &Vec3) -> Vec3{
	v - 2.0 * dot_product(v,n) * n
}

#[allow(dead_code)]
pub fn refract( v : &Vec3, n : &Vec3, theta : f32) -> Option<Vec3> {
	let dt  = dot_product(v,n);
	let discriminant = 1.0 - theta * theta * ( 1.0 - dt * dt);

	if discriminant > 0.0 {
		return Some( theta * ( v  - n * dt) - n * discriminant.sqrt() );
	}

	//print!("no refract");
	return None;
}


impl Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: f32) -> Vec3 {
		Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
	}
}

impl Mul<f32> for &Vec3 {
	type Output = Vec3;

	fn mul(self, other: f32) -> Vec3 {
		Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
	}
}

impl Mul<Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3 {
		Vec3 { x: self * other.x, y: self * other.y, z: self * other.z }
	}
}

impl Mul<&Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, other: &Vec3) -> Vec3 {
		Vec3 { x: self * other.x, y: self * other.y, z: self * other.z }
	}
}


impl Div<f32> for Vec3 {
	type Output = Vec3;

	fn div(self, other: f32) -> Vec3 {
		Vec3 { x: self.x / other, y: self.y / other, z: self.z / other }
	}
}

impl<'a> Div<f32> for &'a Vec3 {
	type Output = Vec3;

	fn div(self, other: f32) -> Vec3 {
		Vec3 { x: self.x / other, y: self.y / other, z: self.z / other }
	}
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x , y: -self.y , z: -self.z  }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x , y: -self.y , z: -self.z  }
    }
}

//-----------------------------------------------------------------------------

pub struct Vec4{
	pub x : f32,
	pub y : f32,
	pub z : f32,
	pub w : f32
}

impl Vec4 {

	#[allow(dead_code)]
	pub fn new (x: f32, y: f32, z: f32, w: f32) -> Vec4 {
		Vec4 {x: x, y: y, z: z, w : w}
	}
}

//-----------------------------------------------------------------------------

pub struct Mat3x3 {
	pub m : [f32; 9]
}

impl Mat3x3{
	#[allow(dead_code)]
	pub fn zero () -> Mat3x3 {
		Mat3x3 {m : [0.0; 9]}
	}

	#[allow(dead_code)]
	pub fn ident () ->Mat3x3 {
		Mat3x3 {m : [1.0, 0.0, 0.0,
		             0.0, 1.0, 0.0,
		             0.0, 0.0, 1.0]}
	}

	#[allow(dead_code)]
	pub fn x_rotation (a : f32) -> Mat3x3{
		let c = a.cos();
		let s = a.sin();
		Mat3x3 {m :
		  [1.0, 0.0, 0.0,
		   0.0,   c,  -s,
		   0.0,   s,   c,]}
	}

	#[allow(dead_code)]
	pub fn y_rotation (a : f32) -> Mat3x3{
		let c = a.cos();
		let s = a.sin();
		Mat3x3 {m :
		  [  c, 0.0,  -s,
		   0.0, 1.0, 0.0,
		     s, 0.0,   c,]}
	}

	#[allow(dead_code)]
	pub fn z_rotation (a : f32) -> Mat3x3{
		let c = a.cos();
		let s = a.sin();
		Mat3x3 {m :
		  [  c,  -s, 0.0,
		     s,   c, 0.0,
		   0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn scale (x : f32, y : f32, z : f32) -> Mat3x3{
		Mat3x3 {m :
		[    x, 0.0, 0.0,
		   0.0,   y, 0.0,
		   0.0, 0.0,   z,]}
	}

	pub fn apply ( &self, inp: &Vec3) -> Vec3{

		Vec3 {
			x : (inp.x * self.m[0] + inp.y * self.m[1] + inp.z * self.m[2]),
			y : (inp.x * self.m[3] + inp.y * self.m[4] + inp.z * self.m[5]),
			z : (inp.x * self.m[6] + inp.y * self.m[7] + inp.z * self.m[8]),
		}
	}
}

impl Add for Mat3x3 {
	type Output = Mat3x3;

	fn add(self, other: Mat3x3) -> Mat3x3 {
		let mut out = Mat3x3::zero();

		for n in 0..9{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl<'a> Add for &'a Mat3x3 {
	type Output = Mat3x3;

	fn add(self, other: &'a Mat3x3) -> Mat3x3 {
		let mut out = Mat3x3::zero();

		for n in 0..9{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl Sub for Mat3x3 {
	type Output = Mat3x3;

	fn sub(self, other: Mat3x3) -> Mat3x3 {
		let mut out = Mat3x3::zero();

		for n in 0..9{
			out.m[n] = self.m[n] - other.m[n];
		}

		return out;
	}
}

impl<'a> Sub for &'a Mat3x3 {
	type Output = Mat3x3;

	fn sub(self, other: &'a Mat3x3) -> Mat3x3 {
		let mut out = Mat3x3::zero();

		for n in 0..9{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl Mul for Mat3x3 {
	type Output = Mat3x3;

	fn mul(self, other: Mat3x3) -> Mat3x3 {
		let mut out = Mat3x3::zero();

		for n in 0..3{
			for m in 0..3{
				let mut s : f32 = 0.0;
				for k in 0..3{
					s += self.m[n + k*3] * other.m[k + m*3];
				}
				out.m[n + m*3] = s
			}
		}

		return out;
	}
}

impl<'a> Mul for &'a Mat3x3 {
	type Output = Mat3x3;

	fn mul(self, other: &'a Mat3x3) -> Mat3x3 {
		let mut out = Mat3x3::zero();

		for n in 0..3{
			for m in 0..3{
				let mut s : f32 = 0.0;
				for k in 0..3{
					s += self.m[n + k*3] * other.m[k + m*3];
				}
				out.m[n + m*3] = s
			}
		}

		return out;
	}
}
//--------------------------------------------------------------
//-----------------------------------------------------------------------------
#[derive( Copy, Clone )]
pub struct Mat4x4 {
	pub m : [f32; 16]
}

impl Mat4x4{
	#[allow(dead_code)]
	pub fn zero () -> Mat4x4 {
		Mat4x4 {m : [0.0; 16]}
	}

	#[allow(dead_code)]
	pub fn ident () ->Mat4x4 {
		Mat4x4 {m : [1.0, 0.0, 0.0, 0.0,
		             0.0, 1.0, 0.0, 0.0,
		             0.0, 0.0, 1.0, 0.0,
		             0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn x_rotation (a : f32) -> Mat4x4{
		let c = a.cos();
		let s = a.sin();
		Mat4x4 {m :
		  [1.0, 0.0, 0.0, 0.0,
		   0.0,   c,  -s, 0.0,
		   0.0,   s,   c, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn y_rotation (a : f32) -> Mat4x4{
		let c = a.cos();
		let s = a.sin();
		Mat4x4 {m :
		  [  c, 0.0,  -s, 0.0,
		   0.0, 1.0, 0.0, 0.0,
		     s, 0.0,   c, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn z_rotation (a : f32) -> Mat4x4{
		let c = a.cos();
		let s = a.sin();
		Mat4x4 {m :
		  [  c,  -s, 0.0, 0.0,
		     s,   c, 0.0, 0.0,
		   0.0, 0.0, 1.0, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn scale (x : f32, y : f32, z : f32) -> Mat4x4{
		Mat4x4 {m :
		[    x, 0.0, 0.0, 0.0,
		   0.0,   y, 0.0, 0.0,
		   0.0, 0.0,   z, 0.0,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn translation (x : f32, y : f32, z : f32) -> Mat4x4{
		Mat4x4 {m :
		[  1.0, 0.0, 0.0,   x,
		   0.0, 1.0, 0.0,   y,
		   0.0, 0.0, 1.0,   z,
		   0.0, 0.0, 0.0, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn retro_proj (d : f32) -> Mat4x4{
		Mat4x4 {m :
		[  1.0, 0.0, 0.0, 0.0,
		   0.0, 1.0, 0.0, 0.0,
		   0.0, 0.0, 1.0, 0.0,
		   0.0, 0.0,   d, 1.0,]}
	}

	#[allow(dead_code)]
	pub fn ortho_proj (
	    left  : f32,  right  : f32,
	    top   : f32,  bottom : f32,
	    near  : f32,  far    : f32) -> Mat4x4
	{

		let xs =  2.0 / (right - left);
		let ys =  2.0 / (top - bottom);
		let zs = -2.0 / (far - near);

		let wx = -(right + left  ) / (right - left  );
		let wy = -(top   + bottom) / (top   - bottom);
		let wz = -(far   + near  ) / (far   - near  );

		Mat4x4 {m :
			[
				  xs, 0.0,  0.0,  0.0,
				0.0,   ys,  0.0,  0.0,
				0.0,  0.0,   zs,  0.0,
				 wx,   wy,   wz,  1.0,
			]
		}

	}

	#[allow(dead_code)]
	pub fn perspective_proj (
	    left  : f32,  right  : f32,
	    top   : f32,  bottom : f32,
	    near  : f32,  far    : f32) -> Mat4x4
	{

		let xs =  2.0 * near / (right - left);
		let ys =  2.0 * near / (top - bottom);

		let wx = -(right + left  ) / (right - left  );
		let wy = -(top   + bottom) / (top   - bottom);
		let wz = -(far    ) / (far   - near  );

		let s = 2.0 * far * near / (far - near);

		Mat4x4 { m :
			[
				  xs,  0.0,  0.0,  0.0,
				 0.0,   ys,  0.0,  0.0,
				  wx,   wy,   wz, -1.0,
				 0.0,  0.0,    s,  0.0,
			]
		}
	}

	#[allow(dead_code)]
	pub fn fov_perspective_proj (
	    fov  : f32,  aspect  : f32,
	    near  : f32,  far    : f32) -> Mat4x4
	{
		let top     =  (fov / 2.0).tan() * near;
		let bottom  = -(fov / 2.0).tan() * near;
		let right   =  (fov / 2.0).tan() * near * aspect;
		let left    = -(fov / 2.0).tan() * near * aspect;

		Mat4x4::perspective_proj(
			left, right,
			top, bottom,
			near, far
		)
	}

	pub fn apply ( &self, inp: &Vec3) -> Vec3{

		let w = inp.x * self.m[12] + inp.y * self.m[13] + inp.z * self.m[14] + self.m[15];
		let inv_v = 1.0 / w;

		Vec3 {
			x : inv_v * (inp.x * self.m[0] + inp.y * self.m[1] + inp.z * self.m[2]  + self.m[3]),
			y : inv_v * (inp.x * self.m[4] + inp.y * self.m[5] + inp.z * self.m[6]  + self.m[7]),
			z : inv_v * (inp.x * self.m[8] + inp.y * self.m[9] + inp.z * self.m[10] + self.m[11]),
		}
	}

	pub fn apply_rotation( &self, inp: &Vec3) -> Vec3{
		Vec3 {
			x : inp.x * self.m[0] + inp.y * self.m[1] + inp.z * self.m[2],
			y : inp.x * self.m[4] + inp.y * self.m[5] + inp.z * self.m[6],
			z : inp.x * self.m[8] + inp.y * self.m[9] + inp.z * self.m[10],
		}
	}

	pub fn inv (&self) -> Mat4x4 {
		let mut inv = [0.0; 16];
		let m = &self.m;
		// : f, det;
		//int i;

		inv[0] =    m[5]  * m[10] * m[15] -
		            m[5]  * m[11] * m[14] -
		            m[9]  * m[6]  * m[15] +
		            m[9]  * m[7]  * m[14] +
		            m[13] * m[6]  * m[11] -
		            m[13] * m[7]  * m[10];

		inv[4] =   -m[4]  * m[10] * m[15] +
		            m[4]  * m[11] * m[14] +
		            m[8]  * m[6]  * m[15] -
		            m[8]  * m[7]  * m[14] -
		            m[12] * m[6]  * m[11] +
		            m[12] * m[7]  * m[10];

		inv[8] =    m[4]  * m[9]  * m[15] -
		            m[4]  * m[11] * m[13] -
		            m[8]  * m[5]  * m[15] +
		            m[8]  * m[7]  * m[13] +
		            m[12] * m[5]  * m[11] -
		            m[12] * m[7]  * m[9];

		inv[12] =  -m[4]  * m[9]  * m[14] +
		            m[4]  * m[10] * m[13] +
		            m[8]  * m[5]  * m[14] -
		            m[8]  * m[6]  * m[13] -
		            m[12] * m[5]  * m[10] +
		            m[12] * m[6]  * m[9];

		inv[1] =   -m[1]  * m[10] * m[15] +
		            m[1]  * m[11] * m[14] +
		            m[9]  * m[2]  * m[15] -
		            m[9]  * m[3]  * m[14] -
		            m[13] * m[2]  * m[11] +
		            m[13] * m[3]  * m[10];

		inv[5] =    m[0]  * m[10] * m[15] -
		            m[0]  * m[11] * m[14] -
		            m[8]  * m[2]  * m[15] +
		            m[8]  * m[3]  * m[14] +
		            m[12] * m[2]  * m[11] -
		            m[12] * m[3]  * m[10];

		inv[9] =   -m[0]  * m[9]  * m[15] +
		            m[0]  * m[11] * m[13] +
		            m[8]  * m[1]  * m[15] -
		            m[8]  * m[3]  * m[13] -
		            m[12] * m[1]  * m[11] +
		            m[12] * m[3]  * m[9];

		inv[13] =   m[0]  * m[9]  * m[14] -
		            m[0]  * m[10] * m[13] -
		            m[8]  * m[1]  * m[14] +
		            m[8]  * m[2]  * m[13] +
		            m[12] * m[1]  * m[10] -
		            m[12] * m[2]  * m[9];

		inv[2] =    m[1]  * m[6] * m[15] -
		            m[1]  * m[7] * m[14] -
		            m[5]  * m[2] * m[15] +
		            m[5]  * m[3] * m[14] +
		            m[13] * m[2] * m[7]  -
		            m[13] * m[3] * m[6];

		inv[6] =   -m[0]  * m[6] * m[15] +
		            m[0]  * m[7] * m[14] +
		            m[4]  * m[2] * m[15] -
		            m[4]  * m[3] * m[14] -
		            m[12] * m[2] * m[7]  +
		            m[12] * m[3] * m[6];

		inv[10] =   m[0]  * m[5] * m[15] -
		            m[0]  * m[7] * m[13] -
		            m[4]  * m[1] * m[15] +
		            m[4]  * m[3] * m[13] +
		            m[12] * m[1] * m[7]  -
		            m[12] * m[3] * m[5];

		inv[14] =  -m[0]  * m[5] * m[14] +
		            m[0]  * m[6] * m[13] +
		            m[4]  * m[1] * m[14] -
		            m[4]  * m[2] * m[13] -
		            m[12] * m[1] * m[6]  +
		            m[12] * m[2] * m[5];

		inv[3] =   -m[1] * m[6] * m[11] +
		            m[1] * m[7] * m[10] +
		            m[5] * m[2] * m[11] -
		            m[5] * m[3] * m[10] -
		            m[9] * m[2] * m[7]  +
		            m[9] * m[3] * m[6];

		inv[7] =    m[0] * m[6] * m[11] -
		            m[0] * m[7] * m[10] -
		            m[4] * m[2] * m[11] +
		            m[4] * m[3] * m[10] +
		            m[8] * m[2] * m[7]  -
		            m[8] * m[3] * m[6];

		inv[11] =  -m[0] * m[5] * m[11] +
		            m[0] * m[7] * m[9]  +
		            m[4] * m[1] * m[11] -
		            m[4] * m[3] * m[9]  -
		            m[8] * m[1] * m[7]  +
		            m[8] * m[3] * m[5];

		inv[15] =   m[0] * m[5] * m[10] -
		            m[0] * m[6] * m[9]  -
		            m[4] * m[1] * m[10] +
		            m[4] * m[2] * m[9]  +
		            m[8] * m[1] * m[6]  -
		            m[8] * m[2] * m[5];

		let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
		/*
		if (det == 0)
			return false;
		*/
		let det = 1.0 / det;

		for i in  0..16 {
			inv[i] = inv[i] * det;
		}

		Mat4x4{m : inv}
	}

	pub fn get_pos(&self) -> Vec3 {
		Vec3{x: self.m[3], y : self.m[7], z : self.m[11]}
	}

	pub fn set_pos(&mut self, pos : Vec3){
		self.m[3] = pos.x;
		self.m[7] = pos.y;
		self.m[11] = pos.z;
	}
}

impl Add for Mat4x4 {
	type Output = Mat4x4;

	fn add(self, other: Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl<'a> Add for &'a Mat4x4 {
	type Output = Mat4x4;

	fn add(self, other: &'a Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl Sub for Mat4x4 {
	type Output = Mat4x4;

	fn sub(self, other: Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] - other.m[n];
		}

		return out;
	}
}

impl<'a> Sub for &'a Mat4x4 {
	type Output = Mat4x4;

	fn sub(self, other: &'a Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..16{
			out.m[n] = self.m[n] + other.m[n];
		}

		return out;
	}
}

impl Mul for Mat4x4 {
	type Output = Mat4x4;

	fn mul(self, other: Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..4{
			for m in 0..4{
				let mut s : f32 = 0.0;
				for k in 0..4{
					s += self.m[n + k*4] * other.m[k + m*4];
				}
				out.m[n + m*4] = s
			}
		}

		return out;
	}
}

impl<'a> Mul for &'a Mat4x4 {
	type Output = Mat4x4;

	fn mul(self, other: &'a Mat4x4) -> Mat4x4 {
		let mut out = Mat4x4::zero();

		for n in 0..4{
			for m in 0..4{
				let mut s : f32 = 0.0;
				for k in 0..4{
					s += self.m[n + k*4] * other.m[k + m*4];
				}
				out.m[n + m*4] = s
			}
		}

		return out;
	}
}
//--------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Ray {
	pub origin    : Vec3,
	pub direction : Vec3
}

impl Ray{

	#[allow(dead_code)]
	pub fn new (origin: &Vec3, direction: &Vec3) -> Ray {
		Ray { origin : origin.clone(), direction : direction.normalized() }
	}

	#[allow(dead_code)]
	pub fn get_origin(&self)    -> Vec3 { self.origin.clone() }

	#[allow(dead_code)]
	pub fn get_direction(&self) -> Vec3 { self.direction.clone() }

	#[allow(dead_code)]
	pub fn get_point (&self, distance : f32) -> Vec3 
	{ 
		&(self.origin) + distance * &(self.direction)
	}

}

//--------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	use std::f32::consts::PI;

	#[test]
	fn test_multiplication_of_two_ident_matrixs_should_be_ident_matrix() {

		let i1 = Mat4x4::ident();
		let i2 = Mat4x4::ident();

		let i3 = &i1 * &i2;
		for n in 0..16{
			assert_eq!(i3.m[n], i1.m[n]);
		}
	}

	#[test]
	fn test_rotation_in_both_sides_on_the_same_angle_should_result_ident_math(){

		let r1 = Mat4x4::x_rotation(0.1);
		let r2 = Mat4x4::x_rotation(-0.1);
		let r3 = &r1 * &r2;

		let i = Mat4x4::ident();
		for n in 0..16{
			let e = r3.m[n] - i.m[n];
			assert!(e.abs()< 0.001);
		}
	}

	#[test]
	fn test_rotation_of_vector_on_90_degreas(){
		let v1 = Vec3::new(0.0, 1.0, 0.0);
		let v2 = Mat4x4::z_rotation(PI / 2.0).apply(&v1);

		assert!((v2.x - -1.0).abs() < 0.001);
		assert!((v2.y -  0.0).abs() < 0.001);
		assert!((v2.y -  0.0).abs() < 0.001);
	}

	#[test]
	fn test_translation (){
		let i1 = Mat4x4::ident();
		let t1 = Mat4x4::translation(0.1, 0.2, 0.3);
		let r = i1 * t1;

		assert!((r.m[0] - 1.0).abs() < 0.001);
		assert!((r.m[1] - 0.0).abs() < 0.001);
		assert!((r.m[2] - 0.0).abs() < 0.001);

		assert!((r.m[3]  - 0.1).abs() < 0.001);
		assert!((r.m[7]  - 0.2).abs() < 0.001);
		assert!((r.m[11] - 0.3).abs() < 0.001);

	}

	#[test]
	fn test_inversion (){
		let i = Mat4x4::ident();
		let inv_i = i.inv();
		for n in 0..16{
			let e = inv_i.m[n] - i.m[n];
			assert!(e.abs()< 0.001);
		}

		let r = Mat4x4::x_rotation(0.1);
		let r_neg = Mat4x4::x_rotation(-0.1);
		let r_inv = r.inv();

		for n in 0..16{
			let e = r_inv.m[n] - r_inv.m[n];
			assert!(e.abs()< 0.001);
		}
	}
}