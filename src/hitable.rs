
use crate::vec_math::*;
use crate::material::*;
use crate::color::*;

pub struct Hit {
	pub distance :f32,
	pub pos : Vec3, //Todo already in scattered
	pub normal : Vec3,
	pub scattered : Ray,
	pub albedo : Color
}

pub trait Hitable { 
	fn hit( &self, ray: &Ray ) -> Option<Hit>;
}


pub struct Sphere {
	pub center : Vec3,
	pub radius : f32,
	pub material : Box<Material>
}

impl Hitable for Sphere
{
	fn hit( &self, ray: &Ray ) -> Option<Hit>
	{
		let oc = &(ray.get_origin()) - &self.center;
		let a = dot_product( &(ray.get_direction()), &(ray.get_direction()));
		let b = dot_product( &oc, &(ray.get_direction()) );
		let c = dot_product( &oc, &oc ) - self.radius * self.radius;
		let discriminant = b * b - a *c;


		if discriminant < 0.0001 {
			return None
		} 

		let temp_1 = ( -b - discriminant.sqrt() ) /  a ;
		let temp_2 = ( -b + discriminant.sqrt() ) /  a ;
		if temp_1 < 0.0001 && temp_2 < 0.0001 {
			return None;
		}

		let distance = if temp_1 >= 0.0001 { temp_1 } else { temp_2 };


		let hit_point = ray.get_point(distance);

		let n = ( &hit_point - &self.center ) / self.radius;

		let (scattered, albedo) = self.material.scatter( &ray.direction, &n, &hit_point,0.0,0.0 );

		return Some( Hit{
			distance,
			pos : hit_point,
			normal: n,
			scattered,
			albedo
		} );
		
	}
}

pub struct Plane {
	pub normal : Vec3,
	pub d : f32,
	pub material : Box<Material>
}

impl Hitable for Plane {
	fn hit( &self, ray: &Ray ) -> Option<Hit>{
		let a = &ray.origin;
		let b = &ray.direction;
		let n = self.normal.clone();

		let den = dot_product( &n, &b );

		if den == 0.0 {
			return None;
		}

		let distance = - ( dot_product(&n, &a) + self.d ) / den;

		if distance < 0.0001 {
			return None;
		}

		let hit_point = ray.get_point(distance);

		let (scattered, albedo) = self.material.scatter( &ray.direction ,&n, &hit_point,hit_point.x,hit_point.z);

		return Some( Hit{
			distance,
			pos : hit_point,
			normal: n,
			scattered,
			albedo
		} );
		
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::f32::consts::PI;

	#[test]
	fn test_() {

		let material = Lambertian { albedo : Color::new (0.0, 0.0, 0.0) };

		{
			let s = Sphere{center : Vec3::new(0.0,0.0,0.0), radius : 1.0, material };
			let ray = Ray::new( &Vec3::new(-2.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0) );

			let (d, p, n,_,_) = s.hit( &ray );

			assert!((d - 1.0).abs()< 0.001);
			assert!((p.x - -1.0).abs()< 0.001);
			assert!((p.y       ).abs()< 0.001);
			assert!((p.z       ).abs()< 0.001);

			assert!((n.x - -1.0).abs()< 0.001);
			assert!((n.y       ).abs()< 0.001);
			assert!((n.z       ).abs()< 0.001);
		}
		{
			let s = Sphere{center : Vec3::new(0.0,0.0,0.0), radius : 0.5, material};
			let ray = Ray::new( &Vec3::new(-1.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0) );

			let (d, p, n) = s.hit( &ray );

			assert!((d - 0.5).abs()< 0.001);
			assert!((p.x - -0.5).abs()< 0.001);
			assert!((p.y       ).abs()< 0.001);
			assert!((p.z       ).abs()< 0.001);

			assert!((n.x - -1.0).abs()< 0.001);
			assert!((n.y       ).abs()< 0.001);
			assert!((n.z       ).abs()< 0.001);
		}
		{
			let s = Sphere{center : Vec3::new(0.0,0.0,0.0), radius : 1.0, material};
			let ray = Ray::new( &Vec3::new(0.0, 2.0, 0.0), &Vec3::new(0.0, -1.0, 0.0) );

			let (d, p, n) = s.hit( &ray );

			assert!((d - 1.0).abs()< 0.001);
			assert!((p.x       ).abs()< 0.001);
			assert!((p.y -  1.0).abs()< 0.001);
			assert!((p.z       ).abs()< 0.001);

			assert!((n.x       ).abs()< 0.001);
			assert!((n.y -  1.0).abs()< 0.001);
			assert!((n.z       ).abs()< 0.001);
		}
		{
			let s = Sphere{center : Vec3::new(0.0, 0.0, 0.0), radius : 1.0, material };
			let ray = Ray::new( &Vec3::new(0.0, 0.0, 2.0), &Vec3::new(0.0, 0.0, -1.0) );

			let (d, p, n) = s.hit( &ray );

			assert!((d - 1.0).abs()< 0.001);
			assert!((p.x       ).abs()< 0.001);
			assert!((p.y       ).abs()< 0.001);
			assert!((p.z -  1.0).abs()< 0.001);

			assert!((n.x       ).abs()< 0.001);
			assert!((n.y       ).abs()< 0.001);
			assert!((n.z -  1.0).abs()< 0.001);
		}

		{
			let s = Sphere{center : Vec3::new(0.0, -0.31, 0.0), radius : 0.3, material};
			let ray = Ray::new( &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, -1.0, 0.0) );

			let (d, p, n) = s.hit( &ray );

			assert!((d - 0.01).abs()< 0.001);
			assert!((p.x       ).abs()< 0.001);
			assert!((p.y - -0.01).abs()< 0.001);
			assert!((p.z       ).abs()< 0.001);

			assert!((n.x       ).abs()< 0.001);
			assert!((n.y -  1.0).abs()< 0.001);
			assert!((n.z       ).abs()< 0.001);
		}


	}
}