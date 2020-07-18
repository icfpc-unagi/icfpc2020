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

pub fn draw(list_of_coords: &Vec<(num::BigInt, num::BigInt)>) {
    println!("----------");

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
    println!("----------");
}

pub fn multidraw(list_of_list_of_coords: &Vec<Vec<(num::BigInt, num::BigInt)>>) {
    for list_of_coords in list_of_list_of_coords {
        draw(list_of_coords);
    }
}

pub fn multidraw_from_e(list_of_list_of_coords: &E) {
    let list_of_list_of_coords = collect_list_of_list_of_coords(list_of_list_of_coords);
    multidraw(&list_of_list_of_coords);
}