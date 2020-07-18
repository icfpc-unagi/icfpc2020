use super::parser::*;
use num::BigInt;

fn collect_coords(e: &E) -> (num::BigInt, num::BigInt) {
    if let E::Pair(a, b) = e {
        if let E::Num(a) = a.as_ref() {
            if let E::Num(b) = b.as_ref() {
               return (a.clone(), b.clone());
            }
        }
    }
    panic!("Coords expected: {}", e);
}

fn collect_list_of_coords(e: &E) -> Vec<(num::BigInt, num::BigInt)> {
    if let Some(list_of_e) = get_list(e) {
        return list_of_e.iter().map(|rce| collect_coords(rce.as_ref())).collect();
    }
    panic!("List of coords expected: {}", e);
}

fn collect_list_of_list_of_coords(e: &E) -> Vec<Vec<(num::BigInt, num::BigInt)>> {
    if let Some(list_of_e) = get_list(e) {
        return list_of_e.iter().map(|rce| collect_list_of_coords(rce.as_ref())).collect();
    }
    panic!("List of list of coords expected: {}", e);
}

fn bigint_to_usize(x: &BigInt) -> usize {
    x.to_string().parse().unwrap()
}

pub fn draw(list_of_coords: &Vec<(num::BigInt, num::BigInt)>, name: &str) {
    println!("---------- {} ----------", name);

    if list_of_coords.len() == 0 {
        println!("(Empty)")
    } else {
        let min_x = list_of_coords.iter().map(|c| c.0.clone()).min().unwrap();
        let max_x = list_of_coords.iter().map(|c| c.0.clone()).max().unwrap();
        let min_y = list_of_coords.iter().map(|c| c.1.clone()).min().unwrap();
        let max_y = list_of_coords.iter().map(|c| c.1.clone()).max().unwrap();
        let w = bigint_to_usize(&(max_x.clone() - min_x.clone())) + 1;
        let h = bigint_to_usize(&(max_y.clone() - min_y.clone())) + 1;

        let mut bitmap = vec![vec![false; w]; h];
        for coord in list_of_coords {
            let x = bigint_to_usize(&(&coord.0.clone() - min_x.clone()));
            let y = bigint_to_usize(&(&coord.1.clone() - min_y.clone()));
            bitmap[y][x] = true;
        }

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
pub fn multidraw_stacked(list_of_list_of_coords: &Vec<Vec<(num::BigInt, num::BigInt)>>) {
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

pub fn multidraw(list_of_list_of_coords: &Vec<Vec<(num::BigInt, num::BigInt)>>) {
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

pub fn draw_from_vec_to_file(list_of_coords: &Vec<(num::BigInt, num::BigInt)>, path: &str) {
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

pub fn multidraw(list_of_list_of_coords: &Vec<Vec<(num::BigInt, num::BigInt)>>) {
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

pub fn draw_from_vec_to_file(list_of_coords: &Vec<(num::BigInt, num::BigInt)>, path: &str) {
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