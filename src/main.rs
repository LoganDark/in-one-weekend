use minifb::{Window, WindowOptions, Key, KeyRepeat};
use std::time::Instant;
use in_one_weekend::math::color::Color;
use in_one_weekend::math::space::{Pos, Vec3};
use ultraviolet::DRotor3;
use in_one_weekend::scene::TestScene;
use in_one_weekend::camera::Camera;
use in_one_weekend::progressive::ProgressiveBuffer;

use in_one_weekend::shape::sphere::Sphere;
use in_one_weekend::shape::plane::{Plane, FinitePlane};
use in_one_weekend::shape::csg::CSG;
use in_one_weekend::material::lambertian::Lambertian;
use in_one_weekend::material::glossy::Glossy;
use in_one_weekend::material::albedo::normals::Normals;
use in_one_weekend::material::emissive::Emissive;

#[cfg(not(debug_assertions))]
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use in_one_weekend::shape::volume::Volume;

fn main() {
	let mut window = Window::new("Thing", 640, 480,
		WindowOptions { resize: true, ..WindowOptions::default() }
	).expect("Couldn't create window");

	//const FPS: u64 = 60;
	//window.limit_update_rate(Some(Duration::from_nanos(1000000000 / FPS)));

	window.limit_update_rate(None);
	window.update();

	let mut scene = TestScene::new();

	scene.objects.push(Box::new(Plane {
		center: Vec3::new(0., 0., 0.),
		normal: Vec3::new(0., 1., 0.),
		material: Box::new(Lambertian::solid(Color(0.1, 0.1, 0.1)))
	}));

	//scene.objects.push(Box::new(Sphere {
	//	center: Vec3::new(-2., 1., 0.),
	//	radius: 1.,
	//	material: Box::new(Lambertian::solid(Color(1., 1., 1.)))
	//}));

	//scene.objects.push(Box::new(Sphere {
	//	center: Vec3::new(0., 1., 0.),
	//	radius: 1.,
	//	material: Box::new(Normals())
	//}));

	//scene.objects.push(Box::new(Sphere {
	//	center: Vec3::new(2., 1., 0.),
	//	radius: 1.,
	//	material: Box::new(Mirror(0.))
	//}));

	//scene.objects.push(Box::new(Plane {
	//	center: Vec3::new(-15., 0., 0.),
	//	normal: Vec3::new(1., 0., 0.),
	//	material: Box::new(Solid(Color(0.9, 0.25, 0.25)))
	//}));

	//scene.objects.push(Box::new(Plane {
	//	center: Vec3::new(15., 0., 0.),
	//	normal: Vec3::new(-1., 0., 0.),
	//	material: Box::new(Solid(Color(0.25, 0.25, 0.9)))
	//}));

	//scene.objects.push(Box::new(FinitePlane {
	//	pos: Pos::new(Vec3::new(-4., 1., 0.), DRotor3::from_euler_angles(0., PI / 2., 0.)),
	//	size: (1., 2.),
	//	material: Box::new(Portal(Pos::new(Vec3::new(4., 0., 2.), DRotor3::identity())))
	//}));

	//for _ in 0..500 {
	//	scene.objects.push(Box::new(Sphere {
	//		center: Vec3::new(
	//			rand::thread_rng().gen_range(-15., 15.),
	//			0.5,
	//			rand::thread_rng().gen_range(-15., 15.)
	//		),
	//		radius: 0.5,
	//		material: Box::new(Normals {})
	//	}));
	//}

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(-3.75, 0.5, 0.),
		radius: 0.5,
		material: Box::new(Lambertian::solid(Color(0.25, 0.25, 1.0)))
	}));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(-2.25, 0.5, 0.),
		radius: 0.5,
		material: Box::new(Emissive::solid(Color::splat(5.)))
	}));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(-0.75, 0.5, 0.),
		radius: 0.5,
		material: Box::new(Lambertian::solid(Color(1.0, 0.25, 0.25)))
	}));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(0.75, 0.5, 0.),
		radius: 0.5,
		material: Box::new(Emissive::solid(Color::splat(5.)))
	}));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(2.25, 0.5, 0.),
		radius: 0.5,
		material: Box::new(Lambertian::solid(Color(0.25, 1.0, 0.25)))
	}));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(3.75, 0.5, 0.),
		radius: 0.5,
		material: Box::new(Emissive::solid(Color::splat(5.)))
	}));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(0., 2., 0.),
		radius: 0.5,
		material: Box::new(Glossy(Normals()))
	}));

	let csg1 = Sphere {
		center: Vec3::new(-3., 2., 0.),
		radius: 0.5,
		material: Box::new(Lambertian(Normals()))
	};

	let csg2 = Sphere {
		center: Vec3::new(-2.5, 2.5, 0.),
		radius: 0.5,
		material: Box::new(Lambertian::solid(Color::splat(0.75)))
	};

	let csghalve = Plane {
		center: Vec3::new(-3., 2., 0.),
		normal: Vec3::new(0., 0., 1.),
		material: Box::new(Lambertian::solid(Color::splat(0.5)))
	};

	scene.objects.push(Box::new(CSG::subtract(
		CSG::union(csg1, csg2),
		csghalve
	)));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(-3., 2., 0.),
		radius: 0.125,
		material: Box::new(Emissive::solid(Color::splat(5.)))
	}));

	scene.objects.push(Box::new(Sphere {
		center: Vec3::new(-2.5, 2.5, 0.),
		radius: 0.125,
		material: Box::new(Emissive::solid(Color::splat(5.)))
	}));

	scene.objects.push(Box::new(Volume(Sphere {
		center: Vec3::new(0., 4., 0.),
		radius: 0.5,
		material: Box::new(Lambertian::solid(Color::splat(1.)))
	}, 1.)));

	//scene.objects.push(Box::new(FinitePlane {
	//	pos: Pos::new(Vec3::new(0., 5., 0.), DRotor3::identity()),
	//	size: (10., 5.),
	//	material: Box::new(Solid(Color(1.0, 1.0, 1.0)))
	//}));

	scene.objects.push(Box::new(FinitePlane {
		pos: Pos::new(Vec3::new(0., 15., 0.), DRotor3::identity()),
		size: (10., 5.),
		material: Box::new(Emissive::solid(Color::splat(5.)))
	}));

	let mut camera_yaw = 0.;
	let mut camera_pitch = 0.;
	let mut camera = Camera {
		pos: Pos::new(Vec3::new(0., 1., 10.), DRotor3::identity()),
		blur_pos: None
	};

	const DEFAULT_DIVIDE: usize = 8;
	const ULTRA_DIVIDE: usize = 1;

	#[cfg(target_os = "macos")]
	const MIN_DIVIDE: usize = 1;

	#[cfg(not(target_os = "macos"))]
	const MIN_DIVIDE: usize = 2;

	let mut divide = DEFAULT_DIVIDE;
	let mut progressive: Option<ProgressiveBuffer> = Some(ProgressiveBuffer::new(0, 0));

	const DEFAULT_EXPOSURE: f64 = 1.;

	let mut exposure_correction = DEFAULT_EXPOSURE;

	let mut denoising = false;
	let device = oidn::Device::new();

	let mut last = Instant::now();

	while window.is_open() {
		let lastlast = last;
		last = Instant::now();
		let diff = (last - lastlast).as_secs_f64();
		let amount = if window.is_key_down(Key::LeftShift) {
			0.5 * diff
		} else if window.is_key_down(Key::LeftCtrl) {
			50. * diff
		} else {
			5. * diff
		};

		println!("{} FPS", 1. / diff);
		window.set_title(format!("Raytracer - {} spp", progressive.as_ref().map(|p| p.spp()).unwrap_or(1u64)).as_str());

		let mut refreshing_progressive = false;

		if window.is_key_down(Key::Up) {
			camera_pitch += amount / 3.;
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::Down) {
			camera_pitch -= amount / 3.;
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::Left) {
			camera_yaw -= amount / 3.;
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::Right) {
			camera_yaw += amount / 3.;
			refreshing_progressive = true;
		}

		camera.pos.rotation = DRotor3::from_euler_angles(0., camera_pitch, camera_yaw);

		let mut movement = Vec3::new(0., 0., 0.);

		if window.is_key_down(Key::W) {
			movement += Vec3::new(0., 0., -amount);
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::S) {
			movement += Vec3::new(0., 0., amount);
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::A) {
			movement += Vec3::new(-amount, 0., 0.);
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::D) {
			movement += Vec3::new(amount, 0., 0.);
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::Q) {
			movement += Vec3::new(0., -amount, 0.);
			refreshing_progressive = true;
		}

		if window.is_key_down(Key::E) {
			movement += Vec3::new(0., amount, 0.);
			refreshing_progressive = true;
		}

		camera.pos.translation += movement.rotated_by(camera.pos.rotation);

		if window.is_key_down(Key::LeftBracket) {
			exposure_correction /= f64::powf(3., diff);
		}

		if window.is_key_down(Key::RightBracket) {
			exposure_correction *= f64::powf(3., diff);
		}

		if window.is_key_down(Key::Backslash) {
			exposure_correction = DEFAULT_EXPOSURE;
		}

		if window.is_key_pressed(Key::Comma, KeyRepeat::No) {
			divide *= 2;
			refreshing_progressive = true;
		}

		if window.is_key_pressed(Key::Period, KeyRepeat::No) {
			if divide > MIN_DIVIDE {
				divide /= 2;
				refreshing_progressive = true;
			}
		}

		if window.is_key_pressed(Key::X, KeyRepeat::No) {
			refreshing_progressive |= divide != ULTRA_DIVIDE;
			divide = ULTRA_DIVIDE;
		}

		if window.is_key_pressed(Key::Z, KeyRepeat::No) {
			refreshing_progressive |= divide != DEFAULT_DIVIDE;
			divide = DEFAULT_DIVIDE;
		}

		let (width, height) = window.get_size();
		let (width, height) = (width * 2 / divide, height * 2 / divide);

		if window.is_key_pressed(Key::P, KeyRepeat::No) {
			match &progressive {
				Some(_) => {
					progressive = None
				}
				None => {
					progressive = Some(ProgressiveBuffer::new(width, height))
				}
			}
		} else if progressive.is_some() && (refreshing_progressive || progressive.as_ref().unwrap().as_ref().len() != width * height) {
			progressive = Some(ProgressiveBuffer::new(width, height))
		}

		if window.is_key_pressed(Key::O, KeyRepeat::No) {
			denoising = !denoising;
		}

		let render;

		let buf = match &mut progressive {
			Some(progressive) => {
				// try to cram as many renders as possible into 1 frame
				// tone mapping and input and stuff is somewhat significant
				let mut num = 0;

				loop {
					let render_start = Instant::now();

					progressive.update(&camera.render_combined(&scene, width, height));
					num += 1;

					let now = Instant::now();
					let render_time = now - render_start;

					if (Instant::now() - last + render_time).as_millis() > 16 {
						break;
					}
				}

				println!("renders per frame: {}", num);
				println!("samples per frame: {}", num * width * height);
				println!("samples per second: {}", ((num * width * height) as f64 / diff) as usize);

				(*progressive).as_ref()
			}
			None => {
				render = camera.render_combined(&scene, width, height);
				&render
			}
		};

		let buf = if denoising {
			let mut filtered = Camera::separate_color_f32(&Camera::separate_color(buf));
			let albedo = Camera::separate_color_f32(&Camera::separate_albedo(buf));
			let normals = Camera::separate_vec3_f32(&Camera::separate_normals(buf));

			oidn::RayTracing::new(&device)
				.hdr(true)
				.image_dimensions(width, height)
				.albedo_normal(&albedo, &normals)
				.filter_in_place(&mut filtered)
				.unwrap();

			Camera::unite_f32_color(&filtered)
		} else {
			Camera::separate_color(buf)
		};

		#[cfg(not(debug_assertions))]
			let iter = buf.into_par_iter();

		#[cfg(debug_assertions)]
			let iter = buf.into_iter();

		let buffer: Vec<u32> = iter
			.map(|c| (c * exposure_correction).tone_map_filmic_hejl2015(1.0).to_srgb())
			.collect();

		window.update_with_buffer(buffer.as_slice(), width, height)
			.expect("Couldn't update window");
	}
}
