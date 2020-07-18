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

pub fn draw(list_of_coords: E) {
    
}

pub fn multidraw(list_of_list_of_coords: E) {

}
