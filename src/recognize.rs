use super::parser::E;

fn detect_chars(bmp: &Vec<Vec<bool>>) {
    let h = bmp.len();
    let w = bmp[0].len();

    let mut usd = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            if usd[y][x] {
                continue;
            }

            if bmp[y][x] {
                continue;
            }
            // println!("YHO");

            // TODO 周囲がからなこと
            let mut k = 1;
            loop {
                if y + k >= h || !bmp[y + k][x] {
                    break;
                }
                if x + k >= w || !bmp[y][x + k] {
                    break;
                }
                k += 1;
            }
            k -= 1;
            // println!("YO: {} {} {}", x, y, k);

            if k <= 0 {
                continue;
            }

            let mut n = num::BigInt::from(0);
            let mut b = num::BigInt::from(1 as i32);
            for i in 0..k * k {
                if bmp[y + 1 + i / k][x + 1 + i % k] {
                    n += &b;
                }
                b *= 2;
            }

            println!("found number: (x={}, y={}) -> {}", x, y, n);

            for dx in 0..=k {
                for dy in 0..=k {
                    usd[y + dy][x + dx] = true;
                }
            }
        }
    }
}

pub fn recognize(e: &E) {
    println!("recognize!");
    let list_of_list_of_coords = super::visualize::collect_list_of_list_of_coords(e);
    for (_i, list_of_coords) in list_of_list_of_coords.iter().enumerate() {
        let bmp = super::visualize::create_bitmap(list_of_coords);
        detect_chars(&bmp);
    }
}

