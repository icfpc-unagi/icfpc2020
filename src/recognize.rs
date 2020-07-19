use super::parser::E;
use std::fmt::Formatter;

const CHARS: &'static [(&'static str, &'static str)] = &[
    ("galaxy", r#"
..###..
.....#.
.###..#
#.#.#.#
#..###.
.#.....
..###..
"#),
    /*
    ("equal", r#"
###
#..
###
"#)*/
];

type Bitmap2D = Vec<Vec<bool>>;

#[derive(Debug, Clone)]
pub enum RecognizedChar {
    Num(num::BigInt),
    Char(String),
}

impl RecognizedChar {
    pub fn starts_with(&self, prefix: &str) -> bool {
        match self {
            RecognizedChar::Num(n) => false,  // TODO やる
            RecognizedChar::Char(name) => name.starts_with(prefix),
        }
    }
}

impl std::fmt::Display for RecognizedChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecognizedChar::Num(n) => write!(f, "{}", n),
            RecognizedChar::Char(name) => write!(f, "{}", name),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Recognizer {
    char_templates: Vec<(&'static str, Bitmap2D)>,
}

pub fn prepare_char_templates() -> Vec<(&'static str, Bitmap2D)> {
    let mut v = vec![];
    for (name, template) in CHARS.iter() {
        let template: Vec<Vec<_>> = template.lines().filter(|line| !line.is_empty()).map(|line| line.chars().map(|c| c == '#').collect()).collect();

        // 横幅が統一されてることチェック
        let w = template[0].len();
        assert!(template.iter().all(|t| t.len() == w));

        v.push((*name, template));
    }
    v
}

impl Recognizer {
    pub fn new() -> Self {
        Self {
            char_templates: prepare_char_templates()
        }
    }

    // [((channel, x, y), result), ...]
    pub fn recognize(&self, e: &E) -> RecognitionResult {
        let list_of_list_of_coords = super::visualize::collect_list_of_list_of_coords(e);

        let mut results = vec![];
        for (channel, list_of_coords) in list_of_list_of_coords.iter().enumerate() {
            let (bmp, (min_x, min_y)) = super::visualize::create_bitmap_with_offset(list_of_coords);
            let match_results = self.match_all(&bmp);

            results.append(&mut match_results.into_iter().map(|((x, y), rr)| ((channel, min_x.clone() + x, min_y.clone() + y), rr)).collect());
        }

        RecognitionResult {
            chars: results
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    fn does_match_char_at(&self, bmp: &Bitmap2D, template: &Bitmap2D, x: usize, y: usize) -> bool {
        // TODO: 周囲が空いていることをチェックする
        let tw = template[0].len();
        let th = template.len();

        if x + tw > bmp[0].len() {
            return false;
        }
        if y + th > bmp.len() {
            return false;
        }

        for dy in 0..th {
            for dx in 0..tw {
                if bmp[y + dy][x + dx] != template[dy][dx] {
                    return false;
                }
            }
        }
        true
    }

    /// 空白であることをチェックする。OKならtrue。
    ///
    /// xとyは左上なので、余白をチェックするのはそのさらに外だよ
    fn check_margin(&self, bmp: &Bitmap2D, x: usize, y: usize, w: usize, h: usize) -> bool {
        // クソだるいのでここだけsignedで処理するよー
        let x = x as i64;
        let y = y as i64;

        for dx in -1..=(w as i64) {
            for &dy in &[-1, h as i64] {
                let tx = x + dx;
                let ty = y + dy;
                if tx < 0 || tx >= (bmp[0].len() as i64) {
                    continue;
                }
                if ty < 0 || ty >= (bmp.len() as i64) {
                    continue;
                }
                if bmp[ty as usize][tx as usize] {
                    return false
                }
            }
        }

        for dy in -1..=(h as i64) {
            for &dx in &[-1, w as i64] {
                let tx = x + dx;
                let ty = y + dy;
                if tx < 0 || tx >= (bmp[0].len() as i64) {
                    continue;
                }
                if ty < 0 || ty >= (bmp.len() as i64) {
                    continue;
                }
                if bmp[ty as usize][tx as usize] {
                    return false
                }
            }
        }

        true
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    // 注意：返してる座標は中央！（クリックしたいのでは的な気持ち）
    fn match_chars_at(&self, bmp: &Vec<Vec<bool>>, x: usize, y: usize) -> Option<((usize, usize), RecognizedChar)> {
        for (name, template) in self.char_templates.iter() {
            if self.does_match_char_at(bmp, template, x, y) {
                let center_x = x + template[0].len() / 2;
                let center_y = y + template[0].len() / 2;
                return Some(((center_x, center_y), RecognizedChar::Char(name.to_string())));
            }
        }
        None
    }

    fn match_num_at(&self, bmp: &Vec<Vec<bool>>, x: usize, y: usize) -> Option<((usize, usize), RecognizedChar)> {
        let h = bmp.len();
        let w = bmp[0].len();

        if bmp[y][x] {
            return None;
        }


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

        if k <= 0 {
            return None
        }

        let mut sgn;
        let center_x = x + (k + 1) / 2;
        let center_y;
        if y + k + 1 < h && bmp[y + k + 1][x] {
            sgn = -1;
            center_y = y + (k + 2) / 2;
            if !self.check_margin(bmp, x, y, k + 1, k + 2) {
                return None;
            }
        } else {
            sgn = 1;
            center_y = y + (k + 1) / 2;
            if !self.check_margin(bmp, x, y, k + 1, k + 1) {
                return None;
            }
        }

        // dbg!(x, y, k);

        let mut n = num::BigInt::from(0);
        let mut b = num::BigInt::from(1 as i32);
        for i in 0..k * k {
            if bmp[y + 1 + i / k][x + 1 + i % k] {
                n += &b;
            }
            b *= 2;
        }
        n *= sgn;

        //println!("found number: (x={}, y={}) -> {}", x, y, n);
        Some(((center_x, center_y), RecognizedChar::Num(n)))
    }

    fn match_at(&self, bmp: &Vec<Vec<bool>>, x: usize, y: usize) -> Option<((usize, usize), RecognizedChar)> {
        self.match_num_at(bmp, x, y).or_else(|| self.match_chars_at(bmp, x, y))
    }

    fn match_all(&self, bmp: &Vec<Vec<bool>>) -> Vec<((usize, usize), RecognizedChar)> {
        let h = bmp.len();
        let w = bmp[0].len();

        let mut results = vec![];
        for y in 0..h {
            for x in 0..w {
                if let Some(((cx, cy), rr)) = self.match_at(bmp, x, y) {
                    results.push(((cx, cy), rr));
                }
            }
        }

        results
    }

    fn detect_chars(&self, bmp: &Vec<Vec<bool>>) {
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

            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RecognitionResult {
    chars: Vec<((usize, num::BigInt, num::BigInt), RecognizedChar)>,
}

impl RecognitionResult {
    pub fn new_empty() -> Self {
        Self {
            chars: vec![]
        }
    }

    pub fn pretty_print(&self) {
        println!("{}", "-".repeat(80));
        println!("{:>5} {:>5} {:>5} | Value", "c", "x", "y");
        println!("{}", "-".repeat(80));
        for row in self.chars.iter() {
            println!("{:>5} {:>5} {:>5} | {}", (row.0).0, (row.0).1, (row.0).2, row.1);
        }
        println!("{}", "-".repeat(80));
    }

    pub fn filter_command(&self, original_command: &str) -> String {
        if original_command.is_empty() {
            return original_command.to_string();
        }

        let mut matches = vec![];
        for ((c, x, y), rc) in self.chars.iter() {
            if rc.starts_with(original_command) {
                matches.push((*c, x.clone(), y.clone()));
            }
        }

        if matches.len() == 0 {
            return original_command.to_string();
        }
        if matches.len() >= 2 {
            eprintln!("Recognizer: multiple matches: {:?}", matches);
            return original_command.to_string();
        }

        // unique match
        let m = &matches[0];
        let command = format!("{} {}", m.1, m.2);
        eprintln!("Recognizer: {} -> {}", original_command, &command);
        command
    }
}