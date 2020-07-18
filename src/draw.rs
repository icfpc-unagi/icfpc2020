use super::parser::*;
use image::*;
use num::*;
use std::vec::Vec;

const W: u32 = 17;
const H: u32 = 13;
const GRADIENT: colorous::Gradient = colorous::TURBO;

pub fn translate_to_vec(e: &E) -> Vec<(BigInt, BigInt)> {
	let mut out = Vec::new();
	for i in e {
		if let E::Pair(x, y) = i {
			if let E::Num(x) = x.as_ref() {
				if let E::Num(y) = y.as_ref() {
					out.push((x.clone(), y.clone()));
				} else {
					eprintln!("unexpected {:?}", y.as_ref());
				}
			} else {
				eprintln!("unexpected {:?}", x.as_ref());
			}
		} else {
			eprintln!("unexpected {:?}", i);
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
	let mut img = DynamicImage::new_luma8(W, H);
	draw_on(&mut img, dots, Rgba([255, 255, 255, 255]));
	img
}

// stack images vertically from top to bottom
pub fn multidraw_stack(v: &Vec<Vec<(BigInt, BigInt)>>) -> DynamicImage {
	let mut img = DynamicImage::new_rgb8(W, H * v.len() as u32);
	for i in 0..v.len() {
		draw_on(
			&mut img.sub_image(0, H * i as u32, W, H),
			&v[i],
			Rgba([255, 255, 255, 255]),
		);
	}
	img
}

// overwrite with gradient colormap
pub fn multidraw_gradient(v: &Vec<Vec<(BigInt, BigInt)>>) -> DynamicImage {
	let mut img = DynamicImage::new_rgb8(W, H);
	for i in 0..v.len() {
		let c = GRADIENT.eval_rational(i, 255);
		draw_on(
			&mut img.sub_image(W * i as u32, 0, W, H),
			&v[i],
			Rgba([c.r, c.g, c.b, 255]),
		);
	}
	img
}

fn draw_on<T: GenericImage<Pixel = Rgba<u8>>>(
	img: &mut T,
	dots: &Vec<(BigInt, BigInt)>,
	px: Rgba<u8>,
) {
	for dot in dots {
		if let Some(x) = dot.0.to_u32() {
			if let Some(y) = dot.1.to_u32() {
				if x < img.width() && y < img.height() {
					img.put_pixel(x, y, px);
				}
			}
		}
	}
}

#[test]
fn test_draw() {
	let img = draw(&bigvecs(&[(1, 2), (-1, -1)]));
	assert_eq!(img.get_pixel(0, 0), &Luma::from([0]));
	assert_eq!(img.get_pixel(1, 2), &Luma::from([255]));
}

#[test]
#[ignore]
fn test_draw_save() {
	let img = draw(&bigvecs(&[(1, 2), (-1, -1), (10, 10)]));
	let tmp = std::env::temp_dir().join("test_draw.png");
	img.save(&tmp).unwrap();
	std::fs::remove_file(&tmp).unwrap();
}

#[cfg(test)]
fn bigvecs(v: &[(i32, i32)]) -> Vec<(BigInt, BigInt)> {
	v.iter()
		.map(|(x, y)| (BigInt::from(*x), BigInt::from(*y)))
		.collect()
}
