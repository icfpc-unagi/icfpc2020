use super::parser::*;
use num::cast::ToPrimitive;
use num::BigInt;

fn collect_coords(e: &E) -> (Int, Int) {
    e.into()
}

pub fn collect_list_of_coords(e: &E) -> Vec<(Int, Int)> {
    e.into_iter().map(|e| collect_coords(e)).collect()
}

pub fn collect_list_of_list_of_coords(e: &E) -> Vec<Vec<(Int, Int)>> {
    e.into_iter().map(|e| collect_list_of_coords(e)).collect()
}

fn bigint_to_usize(x: &BigInt) -> usize {
    x.to_usize().unwrap()
}

pub fn create_bitmap_with_offset(list_of_coords: &Vec<(Int, Int)>) -> (Vec<Vec<bool>>, (Int, Int)) {
    if list_of_coords.len() == 0 {
        (vec![vec![]], (0.into(), 0.into()))
    } else {
        let min_x = list_of_coords.iter().map(|c| c.0.clone()).min().unwrap();
        let max_x = list_of_coords.iter().map(|c| c.0.clone()).max().unwrap();
        let min_y = list_of_coords.iter().map(|c| c.1.clone()).min().unwrap();
        let max_y = list_of_coords.iter().map(|c| c.1.clone()).max().unwrap();
        let w = (max_x - min_x + 1) as usize;
        let h = (max_y - min_y + 1) as usize;

        let mut bitmap = vec![vec![false; w]; h];
        for coord in list_of_coords {
            let x = coord.0 - min_x;
            let y = coord.1 - min_y;
            bitmap[y as usize][x as usize] = true;
        }
        (bitmap, (min_x, min_y))
    }
}

pub fn create_bitmap(list_of_coords: &Vec<(Int, Int)>) -> Vec<Vec<bool>> {
    create_bitmap_with_offset(list_of_coords).0
}

pub fn draw(list_of_coords: &Vec<(Int, Int)>, name: &str) {
    println!("---------- {} ----------", name);

    let bitmap = create_bitmap(list_of_coords);
    if bitmap.is_empty() {
        println!("(Empty)")
    } else {
        for row in &bitmap {
            for b in row {
                print!("{}", if *b { "#" } else { " " })
            }
            println!();
        }
    }
    println!("--------------------");
}

/*
pub fn multidraw_stacked(list_of_list_of_coords: &Vec<Vec<(Int, Int)>>) {
    println!("---------- stacked ----------");

    let min_x = list_of_list_of_coords.iter().map(|loc| loc.iter()).flatten().map(|c| &c.0).min();
    let max_x = list_of_list_of_coords.iter().map(|loc| loc.iter()).flatten().map(|c| &c.0).max();
    let min_y = list_of_list_of_coords.iter().map(|loc| loc.iter()).flatten().map(|c| &c.1).min();
    let max_y = list_of_list_of_coords.iter().map(|loc| loc.iter()).flatten().map(|c| &c.1).max();

    if min_x.is_some() {
        println!("(Empty)")
    } else {
        let min_x = min_x.unwrap();
        let max_x = max_x.unwrap();
        let min_y = min_y.unwrap();
        let max_y = max_y.unwrap();
        let w = bigint_to_usize(&(max_x.clone() - min_x.clone())) + 1;
        let h = bigint_to_usize(&(max_y.clone() - min_y.clone())) + 1;
    }

        /*
    let min_x = list_of_coords.iter().map(|c| c.0.clone()).min().unwrap();
    let max_x = list_of_coords.iter().map(|c| c.0.clone()).max().unwrap();
    let min_y = list_of_coords.iter().map(|c| c.1.clone()).min().unwrap();
    let max_y = list_of_coords.iter().map(|c| c.1.clone()).max().unwrap();
*/

    println!("--------------------");
}
 */

pub fn multidraw(list_of_list_of_coords: &Vec<Vec<(Int, Int)>>) {
    for (i, list_of_coords) in list_of_list_of_coords.iter().enumerate() {
        draw(list_of_coords, &format!("{}", i));
    }
    // multidraw_stacked(list_of_list_of_coords);
}

pub fn multidraw_from_e(list_of_list_of_coords: &E) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    multidraw(&list_of_list_of_coords);
}

pub fn multidraw_stacked_from_e_to_file(list_of_list_of_coords: &E, path: &str) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    let img = super::draw::multidraw_gradient(&list_of_list_of_coords);
    img.save(path).unwrap();
}

pub fn multidraw_stacked_from_e_to_file_scale(list_of_list_of_coords: &E, path: &str, scale: u32) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    let img = super::draw::multidraw_gradient_scale(&list_of_list_of_coords, scale);
    img.save(path).unwrap();
}

pub fn draw_from_vec_to_file(list_of_coords: &Vec<(Int, Int)>, path: &str) {
    let img = super::draw::draw(list_of_coords);
    img.save(path).unwrap();
}

pub fn multidraw_from_e_to_files(list_of_list_of_coords: &E, path_prefix: &str) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    for (i, list_of_coords) in list_of_list_of_coords.iter().enumerate() {
        let path = format!("{}-{}.png", path_prefix, i);
        if list_of_coords.is_empty() {
            println!("Empty image skipped: {}", &path);
        } else {
            draw_from_vec_to_file(list_of_coords, &path);
        }
    }
}

/*

pub fn multidraw(list_of_list_of_coords: &Vec<Vec<(Int, Int)>>) {
    for (i, list_of_coords) in list_of_list_of_coords.iter().enumerate() {
        draw(list_of_coords, &format!("{}", i));
    }
    // multidraw_stacked(list_of_list_of_coords);
}

pub fn multidraw_from_e(list_of_list_of_coords: &E) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    multidraw(&list_of_list_of_coords);
}

pub fn multidraw_stacked_from_e_to_file(list_of_list_of_coords: &E, path: &str) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    let img = super::draw::multidraw_stack(&list_of_list_of_coords);
    img.save(path).unwrap();
}

pub fn draw_from_vec_to_file(list_of_coords: &Vec<(Int, Int)>, path: &str) {
    let img = super::draw::draw(list_of_coords);
    img.save(path).unwrap();
}

pub fn multidraw_from_e_to_file(list_of_list_of_coords: &E, path_prefix: &str) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    for (i, list_of_coords) in list_of_list_of_coords.iter().enumerate() {
        draw_from_vec_to_file(list_of_coords, &format!("{}-{}.png", path_prefix, i));
    }
}
s */
