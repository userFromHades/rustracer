
use crate::vec_math::*;
use crate::random::*;
use crate::color::*;
use crate::texture::*;

use std::rc::Rc;

pub trait Material  { 
	fn scatter( &self, dir: &Vec3, normal: &Vec3, pos: &Vec3, u : f32, v : f32  ) -> (Ray, Color );
}

#[derive(  Clone )]
pub struct Lambertian {
	pub albedo : Rc<Texture>
}

impl Material for Lambertian {
	fn scatter( &self, dir: &Vec3, normal: &Vec3, pos: &Vec3, u : f32, v : f32) -> (Ray, Color )
	{
		let new_dir =  normal + random_in_unit_sphere();
		let new_ray = Ray::new( pos, &new_dir );
		(new_ray, self.albedo.value(u,v))
	}
}

#[derive( Copy, Clone )]
pub struct Metal {
	pub albedo : Color,
	pub fuzz : f32
}

impl Material for Metal {
	fn scatter( &self, dir: &Vec3, normal: &Vec3, pos: &Vec3, u : f32, v : f32  ) -> (Ray, Color )
	{
		let reflected = reflect(dir, normal) +  self.fuzz * random_in_unit_sphere();;
		let new_ray = Ray::new(pos, &reflected );
		(new_ray, self.albedo.clone())
	}
}


#[derive( Copy, Clone )]
pub struct Glass {
	pub albedo : Color,
	pub ref_idx : f32
}

fn schlick(cos :f32, ref_idx : f32) ->f32 {
	let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
	let r0 = r0 * r0;
	r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

impl Material for Glass {
	fn scatter( &self, dir: &Vec3, normal: &Vec3, pos: &Vec3, u : f32, v : f32  ) -> (Ray, Color )
	{

		let (outward, ref_idx, cos) = if dot_product(dir, normal) > 0.0 {
			let cos = dot_product(dir, normal);
			(-normal, self.ref_idx, cos)
		} else {
			let cos = -dot_product(dir, normal);
			(normal.clone(), 1.0 / self.ref_idx, cos)
		};

		let refracted = refract(dir, &outward, ref_idx);
		let prob : f32 = if refracted.is_some() {  schlick (cos, ref_idx) } else { 1.0 };
		
		let new_ray = if random() > prob { Ray::new(pos, &refracted.unwrap() ) } else { Ray::new(pos, &reflect(dir, &outward)  ) };

		(new_ray, self.albedo.clone())
	}
}