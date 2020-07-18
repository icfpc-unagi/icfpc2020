use std::io::prelude::*;

use app::parser::*;
use std::rc::Rc;
use rand::prelude::*;

fn run() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut functions = std::collections::BTreeMap::new();
    for line in stdin.lines() {
        let line = line.unwrap();
        let ss = line.split_whitespace().collect::<Vec<_>>();
        let name = ss[0].to_owned();
        let (exp, n) = parse(&ss[2..], 0);
        assert_eq!(n, ss.len() - 2);
        functions.insert(name, exp);
    }
    // for id in functions.keys() {
    // 	let f = eval(&functions[id], &functions, false);
    // 	println!("{}: {}", id, f);
    // }
    let mut state = E::Etc("nil".to_owned());
    let mut rng = rand::thread_rng();
    let mut data = app::parser::Data::default();

    // let x_array = [0,0,0,0,0,0,0,0,8,2,3,0,-4,9,-4,6,2,1,2];
    // let y_array = [0,0,0,0,0,0,0,0,4,-8,6,-14,10,-3,10,8,-2,79990,3];
    let x_array = [0,0,0,0,0,0,0,0,8,2,3,0,-4,9,-4,6,2,1,2,1,2,1,-1,1,3,1,1,1,-1,1,-15];
    let y_array = [0,0,0,0,0,0,0,0,4,-8,6,-14,10,-3,10,8,-2,79969,3,79959,0,79949,-2,79939,-2,79929,1,79919,-1,79909,1];

    for iter in 0..x_array.len() {
        let x = if iter < x_array.len() { x_array[iter] } else { rng.gen_range(-20, 20) };
        let y = if iter < y_array.len() { y_array[iter] } else { rng.gen_range(-20, 20) };
        // let s = format!("ap ap cons {} {}", x, y);
        let s = if x <= 70000 {format!("ap ap cons {} {}", x, y)}  else {format!("ap ap cons {} ap ap cons {} nil", x, y)};

        let xy = parse(&s.split_whitespace().collect::<Vec<_>>(), 0).0;
        let exp = E::Ap(
            Rc::new(E::Ap(Rc::new(E::Etc(":1338".to_owned())), state.clone().into())),
            xy.into(),
        );
        let f = eval(&exp, &functions, false, &mut data);
        let f = eval(&f, &functions, true, &mut data);
        // eprintln!("{}", f);
        // for (id, c) in data.count {
        // 	eprintln!("{}: {}", id, c);
        // }
        let (flag, new_state, data) = if let E::Pair(flag, a) = f {
            if let E::Pair(a, b) = a.as_ref() {
                if let E::Pair(data, _) = b.as_ref() {
                    (flag.as_ref() != &E::Num(0.into()), a.as_ref().clone(), data.as_ref().clone())
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        };
        if flag || state.to_string() == "[]" || (state.to_string().len(), state.to_string()) < (new_state.to_string().len(), new_state.to_string()) {
            state = new_state;
            if !flag {
                println!("iteration {}", iter);
                app::visualize::multidraw_from_e(&data);
                app::visualize::multidraw_stacked_from_e_to_file_scale(&data, &format!("out/stacked-{}.png", iter), 8);
                app::visualize::multidraw_from_e_to_files(&data, &format!("out/separate-{}", iter));
            }
            eprintln!("{} {}", x, y);
            eprintln!("state: {}", state);
            println!("modulated: {}", app::modulation::modulate(&data));
        }
        if flag {
            break;
        }
    }
    // let f = eval(&functions["hoge"], &functions, false);
    // let f = eval(&f, &functions, true);
    // println!("ret: {}", f);
    // let g = eval(&functions[":1108"], &functions);
    // let g = eval(&functions["galaxy"], &functions);
    // println!("{}", g);
    // let mut f = functions[":1141"].clone();
    // for _ in 0..100 {
    // 	f = simplify(&f);
    // }
    // println!("{}", f);
}

fn main() {
    let _ = ::std::thread::Builder::new()
        .name("run".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join();
}
