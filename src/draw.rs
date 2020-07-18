use super::parser::*;
use image::*;
use num::*;
use std::vec::Vec;

const GRADIENT: colorous::Gradient = colorous::TURBO;

pub fn translate_to_vec(e: &E) -> Vec<(BigInt, BigInt)> {
	let mut out = Vec::new();
	for i in e {
		if let E::Pair(x, y) = i {
			if let E::Num(x) = x.as_ref() {
				if let E::Num(y) = y.as_ref() {
					out.push((x.clone(), y.clone()));
				} else {
					eprintln!("expected Num but got {:?}", y.as_ref());
				}
			} else {
				eprintln!("expected Num but got {:?}", x.as_ref());
			}
		} else {
			eprintln!("expected Pair but got {:?}", i);
		}
	}
	out
}

pub fn translate_to_vecvec(e: &E) -> Vec<Vec<(BigInt, BigInt)>> {
	let mut out = Vec::new();
	for i in e {
		out.push(translate_to_vec(i));
	}
	out
}

pub fn draw(dots: &Vec<(BigInt, BigInt)>) -> DynamicImage {
	let ((w, h), offset) = range_v(dots);
	let mut img = DynamicImage::new_luma8(w, h);
	draw_on(&mut img, dots, &offset, Rgba([255, 255, 255, 255]));
	img
}

// stack images vertically from top to bottom
pub fn multidraw_stack(v: &Vec<Vec<(BigInt, BigInt)>>) -> DynamicImage {
	let ((w, h), offset) = range_vv(v);
	let mut img = DynamicImage::new_rgb8(w, h);
	draw_axes(&mut img, &offset);
	for i in 0..v.len() {
		draw_on(
			&mut img.sub_image(0, h * i as u32, w, h),
			&v[i],
			&offset,
			Rgba([255, 255, 255, 255]),
		);
	}
	img
}

// overwrite with gradient colormap
pub fn multidraw_gradient(v: &Vec<Vec<(BigInt, BigInt)>>) -> DynamicImage {
	let ((w, h), offset) = range_vv(v);
	let mut img = DynamicImage::new_rgb8(w, h);
	draw_axes(&mut img, &offset);
	for i in 0..v.len() {
		let c = GRADIENT.eval_rational(i, 255);
		draw_on(&mut img, &v[i], &offset, Rgba([c.r, c.g, c.b, 255]));
	}
	img
}

fn draw_axes(img: &mut DynamicImage, offset: &(BigInt, BigInt)) {
	if let Some(y) = offset.1.to_u32() {
		if y < img.height() {
			for x in 0..img.width() {
				img.put_pixel(x, y, Rgba([64, 64, 64, 255]));
			}
		}
	}
	if let Some(x) = offset.0.to_u32() {
		if x < img.width() {
			for y in 0..img.height() {
				img.put_pixel(x, y, Rgba([64, 64, 64, 255]));
			}
		}
	}
}

fn range_v(v: &Vec<(BigInt, BigInt)>) -> ((u32, u32), (BigInt, BigInt)) {
	let (min_x, min_y, max_x, max_y) = (
		v.iter().map(|c| &c.0).min().unwrap(),
		v.iter().map(|c| &c.0).max().unwrap(),
		v.iter().map(|c| &c.1).min().unwrap(),
		v.iter().map(|c| &c.1).max().unwrap(),
	);
	(
		(
			(max_x - min_x).to_u32().unwrap() + 1,
			(max_y - min_y).to_u32().unwrap() + 1,
		),
		(-min_x, -min_y),
	)
}
fn range_vv(vv: &Vec<Vec<(BigInt, BigInt)>>) -> ((u32, u32), (BigInt, BigInt)) {
	let (min_x, min_y, max_x, max_y) = (
		vv.iter()
			.map(|loc| loc.iter())
			.flatten()
			.map(|c| &c.0)
			.min()
			.unwrap(),
		vv.iter()
			.map(|loc| loc.iter())
			.flatten()
			.map(|c| &c.0)
			.max()
			.unwrap(),
		vv.iter()
			.map(|loc| loc.iter())
			.flatten()
			.map(|c| &c.1)
			.min()
			.unwrap(),
		vv.iter()
			.map(|loc| loc.iter())
			.flatten()
			.map(|c| &c.1)
			.max()
			.unwrap(),
	);
	(
		(
			(max_x - min_x).to_u32().unwrap() + 1,
			(max_y - min_y).to_u32().unwrap() + 1,
		),
		(-min_x, -min_y),
	)
}

fn draw_on<T: GenericImage<Pixel = Rgba<u8>>>(
	img: &mut T,
	dots: &Vec<(BigInt, BigInt)>,
	offset: &(BigInt, BigInt),
	px: Rgba<u8>,
) {
	for dot in dots {
		if let Some(x) = (&dot.0 + &offset.0).to_u32() {
			if let Some(y) = (&dot.1 + &offset.1).to_u32() {
				// if x < img.width() && y < img.height() {
				img.put_pixel(x, y, px);
				// }
			}
		}
	}
}

// #[test]
// fn test_draw() {
// 	let img = draw(&bigvecs(&[(1, 2), (-1, -1)]));
// 	assert_eq!(img.get_pixel(0, 0), &Luma::from([0]));
// 	assert_eq!(img.get_pixel(1, 2), &Luma::from([255]));
// }

// #[test]
// #[ignore]
// fn test_draw_save() {
// 	let img = draw(&bigvecs(&[(1, 2), (-1, -1), (10, 10)]));
// 	let tmp = std::env::temp_dir().join("test_draw.png");
// 	img.save(&tmp).unwrap();
// 	std::fs::remove_file(&tmp).unwrap();
// }

// #[cfg(test)]
// fn bigvecs(v: &[(i32, i32)]) -> Vec<(BigInt, BigInt)> {
// 	v.iter()
// 		.map(|(x, y)| (BigInt::from(*x), BigInt::from(*y)))
// 		.collect()
// }

// cargo run --release --bin parser_iwiwi < ~/Dropbox/ICFPC2020/galaxy.txt