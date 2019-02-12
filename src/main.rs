
extern crate sdl2;
extern crate rand;

#[macro_use]
extern crate bitflags;

mod canvas;
mod vec_math;
mod hitable;
mod color;
mod random;
mod material;
mod texture;

use self::vec_math::*;
use self::hitable::*;
use self::material::*;
use self::color::*;
use self::random::*;
use self::texture::*;

use std::rc::Rc;


fn find_colour (ray : &Ray, objects: & Vec<Box<Hitable>>, 
                depth : i32) -> Color
{

	if depth > 15 {
		return Color::new ( 0.0, 0.0, 0.0 );
	}

	let mut hit : Option<Hit> = None;
	for h in objects {
		let h = h.hit( &ray );
		if h.is_none(){
			continue;
		}

		match hit {
			Some(hit_) => {
				let h = h.unwrap();
				if hit_.distance > h.distance{ 
					hit = Some(h) ;
				} else {
					hit = Some(hit_);
				}},
			None =>  hit = h
		}

	}

	let colour = match hit {
		Some(hit) => {

			let new_ray = hit.scattered;
			let albedo = hit.albedo;
			let cn = find_colour(&new_ray, &objects, depth + 1 );
			let cn = Color::new ( cn.r * albedo.r, cn.g * albedo.g, cn.b * albedo.b );

			cn
		},
		None => {
			let unit_direction = ray.get_direction();
			let t = 0.5 + unit_direction.y;
			let cn = Color::new (0.8,0.6,0.55) * (1.0 - t) + Color::new (0.7,0.8,1.0) * t;

			cn
		}
	};
	return colour;
}

//-----------------------------------------------------------------------------
fn main() {

	let mut c = canvas::MyCanvas::new (800, 600);

	c.clear();

	let origin = Vec3::new ( 0.0, 0.0, 0.0 );

	let low_left_corner = Vec3::new( 0.0, 0.0, 1.0 );
	let vertical        = Vec3::new( 0.0, 1.0, 0.0 );
	let horizontal      = Vec3::new( 1.0, 0.0, 0.0 );

	let material = Box::new( Lambertian { albedo : Rc::new(ConstantTexture{  color : Color::new (0.5, 0.5, 0.5) }) } );
	let metal_1 = Box::new( Metal {albedo : Color::new (0.5,0.5,0.6), fuzz : 0.1});
	let metal_2 = Box::new( Metal {albedo : Color::new (0.8,0.95,0.75), fuzz : 0.01});

	let glass = Box::new( Glass {albedo : Color::new (0.95,0.95,1.0), ref_idx : 1.5});

	let mut objects: Vec<Box<Hitable>> = Vec::new();
	objects.push(Box::new( Sphere{ center : Vec3::new(  0.5, -0.6, 5.0 ) , radius :0.3, material : material.clone() }));
	objects.push(Box::new( Sphere{ center : Vec3::new( -0.6, 0.0, 1.9 ) , radius :0.15, material : glass.clone() }));
	objects.push(Box::new( Sphere{ center : Vec3::new( -0.8, 0.0, 3.0 ) , radius :0.9, material : material.clone() }));
	objects.push(Box::new( Sphere{ center : Vec3::new(  0.5, 0.0, 3.0 ) , radius :0.3, material : metal_1.clone() }));
	objects.push(Box::new( Sphere{ center : Vec3::new(  0.4, -0.8, 2.0 ) , radius :0.1, material : metal_1.clone() }));
	objects.push(Box::new( Sphere{ center : Vec3::new(  0.3, -0.7, 2.3 ) , radius :0.15, material : glass }));

	let plane_material = Box::new( Lambertian { albedo : Rc::new( ChessTexture{color_a : Color::new (0.0, 0.0, 0.0), color_b :Color::new (1.0, 1.0, 1.0), scale : 1.0}) } );
	objects.push(Box::new( Plane{ normal : Vec3::new(  0.0, 1.0, 0.0 ) , d :0.9, material : plane_material }));

	for x in 0..800 {
		for y in 0..600{

			let mut color_accm = Color::new  (0.0, 0.0, 0.0);

			let smpl = 200;
			let rfactor = 1.1;

			for _ in 0..smpl {

				let u = (x - 400) as f32 / 800.0 + rfactor * random()/800.0;
				let v = (300 - y) as f32 / 600.0 + rfactor * random()/600.0;

				let dir = &low_left_corner + (&vertical * v) + (&horizontal * u);
				let ray = Ray::new(&origin, &dir );

				let colour = find_colour (&ray, &objects, 0);

				color_accm = &color_accm + colour;
			}

			let k = 1.0 / ( smpl as f32 );
			color_accm =  k * &color_accm ; // todo override oppertor

			c.point( x, y, color_accm.as_u32() );
		}
		if x % 20 == 0 {
			c.present();
			c.poll_events();
			println!("{:?}", x);
		}
	}

	c.present();
	c.wait_end();

}
