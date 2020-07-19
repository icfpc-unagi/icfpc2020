use super::parser::*;
use image::*;
use num::*;
use std::vec::Vec;

const GRADIENT: colorous::Gradient = colorous::TURBO;

pub fn translate_to_vec(e: &E) -> Vec<(BigInt, BigInt)> {
	e.into_iter().map(|e| e.into()).collect()
}

pub fn translate_to_vecvec(e: &E) -> Vec<Vec<(BigInt, BigInt)>> {
	e.into_iter().map(|e| translate_to_vec(&e)).collect()
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
	let mut img = DynamicImage::ImageRgb8(RgbImage::from_pixel(w, h, Rgb([255, 255, 255])));
	draw_axes(&mut img, &offset, 10);
	for i in 0..v.len() {
		draw_on(
			&mut img.sub_image(0, h * i as u32, w, h),
			&v[i],
			&offset,
			Rgba([0, 0, 0, 255]),
		);
	}
	img
}

// overwrite with gradient colormap
pub fn multidraw_gradient(v: &Vec<Vec<(BigInt, BigInt)>>) -> DynamicImage {
	let ((w, h), offset) = range_vv(v);
	let mut img = DynamicImage::ImageRgb8(RgbImage::from_pixel(w, h, Rgb([255, 255, 255])));
	draw_axes(&mut img, &offset, 10);
	for i in 0..v.len() {
		let i = v.len() - 1 - i; // overwrite in reverse order
		let c = GRADIENT.eval_rational(i + 1, v.len() + 1);
		draw_on(&mut img, &v[i], &offset, Rgba([c.r, c.g, c.b, 255]));
	}
	img
}

pub fn multidraw_gradient_scale(v: &Vec<Vec<(BigInt, BigInt)>>, scale: u32) -> DynamicImage {
	let ((w, h), offset) = range_vv(v);
	let mut img = DynamicImage::ImageRgb8(RgbImage::from_pixel(
		w * scale,
		h * scale,
		Rgb([255, 255, 255]),
	));
	draw_axes(
		&mut img,
		&(&offset.0 * scale, &offset.1 * scale),
		10 * scale,
	);
	for i in 0..v.len() {
		let i = v.len() - 1 - i; // overwrite in reverse order
		let c = GRADIENT.eval_rational(i + 1, v.len() + 1);
		draw_on_scale(&mut img, &v[i], &offset, Rgba([c.r, c.g, c.b, 255]), scale);
	}
	img
}

fn draw_axes(img: &mut DynamicImage, offset: &(BigInt, BigInt), step: u32) {
	const AXES_COLOR: Rgba<u8> = Rgba([255, 0, 0, 255]);
	const GRID_COLOR: Rgba<u8> = Rgba([127, 127, 127, 255]);
	if let Some(ya) = offset.1.to_u32() {
		for y in (ya % step..img.height()).step_by(step as usize) {
			for x in 0..img.width() {
				img.put_pixel(x, y, if y == ya { AXES_COLOR } else { GRID_COLOR });
			}
		}
	}
	if let Some(xa) = offset.0.to_u32() {
		for x in (xa % step..img.width()).step_by(step as usize) {
			for y in 0..img.height() {
				img.put_pixel(x, y, if x == xa { AXES_COLOR } else { GRID_COLOR });
			}
		}
	}
}

fn range_v(v: &Vec<(BigInt, BigInt)>) -> ((u32, u32), (BigInt, BigInt)) {
	let (min_x, max_x, min_y, max_y) = (
		v.iter().map(|c| &c.0).min().unwrap(),
		v.iter().map(|c| &c.0).max().unwrap(),
		v.iter().map(|c| &c.1).min().unwrap(),
		v.iter().map(|c| &c.1).max().unwrap(),
	);
	(
		(
			(max_x - min_x).to_u32().unwrap() + 3,
			(max_y - min_y).to_u32().unwrap() + 3,
		),
		(1 - min_x, 1 - min_y),
	)
}
pub fn range_vv(vv: &Vec<Vec<(BigInt, BigInt)>>) -> ((u32, u32), (BigInt, BigInt)) {
	let it = vv.iter().map(|loc| loc.iter()).flatten();
	let (min_x, max_x, min_y, max_y) = (
		it.clone().map(|c| &c.0).min().unwrap(),
		it.clone().map(|c| &c.0).max().unwrap(),
		it.clone().map(|c| &c.1).min().unwrap(),
		it.clone().map(|c| &c.1).max().unwrap(),
	);
	(
		(
			(max_x - min_x).to_u32().unwrap() + 3,
			(max_y - min_y).to_u32().unwrap() + 3,
		),
		(1 - min_x, 1 - min_y),
	)
}

fn draw_on<T: GenericImage<Pixel = Rgba<u8>>>(
	img: &mut T,
	dots: &Vec<(BigInt, BigInt)>,
	offset: &(BigInt, BigInt),
	px: Rgba<u8>,
) {
	for dot in dots {
		if let (Some(x), Some(y)) = ((&dot.0 + &offset.0).to_u32(), (&dot.1 + &offset.1).to_u32()) {
			img.put_pixel(x, y, px);
		}
	}
}

fn draw_on_scale<T: GenericImage<Pixel = Rgba<u8>>>(
	img: &mut T,
	dots: &Vec<(BigInt, BigInt)>,
	offset: &(BigInt, BigInt),
	px: Rgba<u8>,
	scale: u32,
) {
	for dot in dots {
		if let (Some(x), Some(y)) = ((&dot.0 + &offset.0).to_u32(), (&dot.1 + &offset.1).to_u32()) {
			for xs in 0..scale {
				for ys in 0..scale {
					img.put_pixel(x * scale + xs, y * scale + ys, px);
				}
			}
		}
	}
}

#[test]
fn test_draw() {
	let img = draw(&bigvecs(&[(1, 2), (-1, -1)]));
	assert_eq!(img.get_pixel(2, 2), Rgba([0, 0, 0, 255]));
	assert_eq!(img.get_pixel(3, 4), Rgba([255, 255, 255, 255]));
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

// cargo run --release --bin parser_iwiwi < ~/Dropbox/ICFPC2020/galaxy.txt
